use std::time::Duration;

use argon2::{
    Argon2,
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
        rand_core::{OsRng, RngCore},
    },
};
use axum::{
    Json, Router,
    extract::{FromRequestParts, State},
    http::{StatusCode, header, request::Parts},
    routing::{get, post},
};
use mongodb::{
    Database, IndexModel,
    bson::{DateTime, Document, doc, oid::ObjectId},
    error::{ErrorKind, WriteFailure},
    options::IndexOptions,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

use crate::{ApiError, ApiResult, App};

const SESSION_AGE: u64 = 7 * 24 * 60 * 60;

#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    #[serde(rename(deserialize = "_id"), serialize_with = "serialize_id")]
    pub id: ObjectId,
    pub username: String,
}

fn serialize_id<S: serde::Serializer>(id: &ObjectId, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&id.to_hex())
}

#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
    email: Option<String>,
}

#[derive(Deserialize)]
struct Account {
    #[serde(rename = "_id")]
    id: ObjectId,
    username: String,
    password_hash: Option<String>,
    hash: Option<String>,
    salt: Option<String>,
}

#[derive(Deserialize)]
struct Session {
    user: ObjectId,
}

#[derive(Serialize)]
struct SignedIn {
    user: User,
    token: String,
    expires_in: u64,
}

pub fn routes() -> Router<App> {
    Router::new()
        .route("/auth/signup", post(signup))
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/me", get(me))
}

pub async fn ensure_indexes(db: &Database) -> mongodb::error::Result<()> {
    db.collection::<Account>("users")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "username": 1 })
                .options(IndexOptions::builder().unique(true).build())
                .build(),
        )
        .await?;
    db.collection::<Session>("rust_sessions")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "expires": 1 })
                .options(IndexOptions::builder().expire_after(Duration::ZERO).build())
                .build(),
        )
        .await?;
    Ok(())
}

fn token_key(parts: &Parts) -> ApiResult<String> {
    let token = parts
        .headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|token| token.len() == 64 && token.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .ok_or_else(unauthorized)?;
    Ok(hex::encode(Sha256::digest(token.as_bytes())))
}

impl FromRequestParts<App> for User {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &App) -> ApiResult<Self> {
        let key = token_key(parts)?;
        let session = state
            .db
            .collection::<Session>("rust_sessions")
            .find_one(doc! { "_id": key, "expires": { "$gt": DateTime::now() } })
            .await?
            .ok_or_else(unauthorized)?;
        state
            .db
            .collection::<User>("users")
            .find_one(doc! { "_id": session.user })
            .await?
            .ok_or_else(unauthorized)
    }
}

fn unauthorized() -> ApiError {
    ApiError(StatusCode::UNAUTHORIZED, "Please sign in to continue.")
}

async fn issue_session(state: &App, user: User) -> ApiResult<Json<SignedIn>> {
    let mut random = [0u8; 32];
    OsRng
        .try_fill_bytes(&mut random)
        .map_err(ApiError::internal)?;
    let token = hex::encode(random);
    state.db.collection("rust_sessions").insert_one(doc! {
        "_id": hex::encode(Sha256::digest(token.as_bytes())),
        "user": user.id,
        "expires": DateTime::from_millis(DateTime::now().timestamp_millis() + SESSION_AGE as i64 * 1000),
    }).await?;
    Ok(Json(SignedIn {
        user,
        token,
        expires_in: SESSION_AGE,
    }))
}

fn hash_password(password: &str) -> ApiResult<String> {
    Argon2::default()
        .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
        .map(|hash| hash.to_string())
        .map_err(ApiError::internal)
}

async fn signup(
    State(state): State<App>,
    Json(input): Json<Credentials>,
) -> ApiResult<Json<SignedIn>> {
    let username = input.username.trim().to_owned();
    let email = input.email.as_deref().unwrap_or_default().trim();
    if !(3..=32).contains(&username.len())
        || !username
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"_-".contains(&byte))
    {
        return Err(ApiError::bad(
            "Use 3 to 32 letters, numbers, hyphens, or underscores for your username.",
        ));
    }
    if email.len() > 254
        || email.chars().any(char::is_whitespace)
        || !email.split_once('@').is_some_and(|(local, domain)| {
            !local.is_empty()
                && !domain.contains('@')
                && domain
                    .split_once('.')
                    .is_some_and(|(a, b)| !a.is_empty() && !b.is_empty())
        })
    {
        return Err(ApiError::bad("Enter a valid email address."));
    }
    if !(12..=256).contains(&input.password.len()) {
        return Err(ApiError::bad("Use a password between 12 and 256 bytes."));
    }
    let hash = tokio::task::spawn_blocking(move || hash_password(&input.password))
        .await
        .map_err(ApiError::internal)??;
    let id = ObjectId::new();
    state
        .db
        .collection::<Document>("users")
        .insert_one(doc! {
            "_id": id, "username": &username, "email": email, "password_hash": hash
        })
        .await
        .map_err(|error| {
            if matches!(error.kind.as_ref(), ErrorKind::Write(WriteFailure::WriteError(error)) if error.code == 11000) {
                ApiError(StatusCode::CONFLICT, "That username is already taken.")
            } else {
                error.into()
            }
        })?;
    issue_session(&state, User { id, username }).await
}

fn legacy_password_matches(password: &str, hash: &str, salt: &str) -> bool {
    let Ok(expected) = hex::decode(hash) else {
        return false;
    };
    if expected.len() != 512 {
        return false;
    }
    let mut derived = [0u8; 512];
    pbkdf2::pbkdf2_hmac::<Sha256>(password.as_bytes(), salt.as_bytes(), 25_000, &mut derived);
    bool::from(derived.as_slice().ct_eq(&expected))
}

async fn login(
    State(state): State<App>,
    Json(input): Json<Credentials>,
) -> ApiResult<Json<SignedIn>> {
    let invalid = || {
        ApiError(
            StatusCode::UNAUTHORIZED,
            "The username or password is incorrect.",
        )
    };
    if input.username.len() > 512 || input.password.len() > 4096 {
        return Err(invalid());
    }

    let account = state
        .db
        .collection::<Account>("users")
        .find_one(doc! { "username": input.username })
        .await?;
    let (user, upgraded) = tokio::task::spawn_blocking(move || {
        let Some(account) = account else {
            hash_password(&input.password)?;
            return Err(invalid());
        };
        let user = User {
            id: account.id,
            username: account.username,
        };
        if let Some(hash) = account.password_hash {
            let parsed = PasswordHash::new(&hash).map_err(ApiError::internal)?;
            Argon2::default()
                .verify_password(input.password.as_bytes(), &parsed)
                .map_err(|_| invalid())?;
            return Ok((user, None));
        }
        let matches = account
            .hash
            .zip(account.salt)
            .is_some_and(|(hash, salt)| legacy_password_matches(&input.password, &hash, &salt));
        if !matches {
            return Err(invalid());
        }
        Ok((user, Some(hash_password(&input.password)?)))
    })
    .await
    .map_err(ApiError::internal)??;

    if let Some(hash) = upgraded {
        state
            .db
            .collection::<Document>("users")
            .update_one(
                doc! { "_id": user.id, "password_hash": { "$exists": false } },
                doc! { "$set": { "password_hash": hash }, "$unset": { "hash": "", "salt": "" } },
            )
            .await?;
    }
    issue_session(&state, user).await
}

async fn me(user: User) -> Json<User> {
    Json(user)
}

async fn logout(
    State(state): State<App>,
    request: axum::extract::Request,
) -> ApiResult<StatusCode> {
    if let Ok(key) = token_key(&request.into_parts().0) {
        state
            .db
            .collection::<Session>("rust_sessions")
            .delete_one(doc! { "_id": key })
            .await?;
    }
    Ok(StatusCode::NO_CONTENT)
}
