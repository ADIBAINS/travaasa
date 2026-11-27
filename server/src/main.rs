mod accounts;
mod listings;

use std::{env, time::Duration};

use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use mongodb::{Client, Database, bson::doc};
use serde_json::json;

#[derive(Clone)]
struct App {
    db: Database,
    http: reqwest::Client,
    mapbox_token: Option<String>,
    cloud: Option<listings::Cloud>,
}

struct ApiError(StatusCode, &'static str);

type ApiResult<T> = Result<T, ApiError>;

impl ApiError {
    fn bad(message: &'static str) -> Self {
        Self(StatusCode::BAD_REQUEST, message)
    }

    fn missing() -> Self {
        Self(StatusCode::NOT_FOUND, "Stay not found.")
    }

    fn internal(error: impl std::fmt::Display) -> Self {
        eprintln!("{error}");
        Self(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to complete the request.",
        )
    }
}

impl From<mongodb::error::Error> for ApiError {
    fn from(error: mongodb::error::Error) -> Self {
        Self::internal(error)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, Json(json!({ "message": self.1 }))).into_response()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let uri =
        env::var("ATLASDB_URL").unwrap_or_else(|_| "mongodb://127.0.0.1:27017/travaasa".into());
    let client = Client::with_uri_str(&uri).await?;
    let db = match env::var("MONGODB_DATABASE") {
        Ok(name) => client.database(&name),
        Err(_) => client
            .default_database()
            .unwrap_or_else(|| client.database("test")),
    };

    db.run_command(doc! { "ping": 1 }).await?;
    accounts::ensure_indexes(&db).await?;
    listings::ensure_indexes(&db).await?;
    let state = App {
        db,
        http: reqwest::Client::builder()
            .timeout(Duration::from_secs(20))
            .build()?,
        mapbox_token: env::var("MAPBOX_TOKEN")
            .ok()
            .filter(|token| !token.is_empty()),
        cloud: listings::Cloud::from_env()?,
    };
    let app = Router::new()
        .route("/health", get(health))
        .nest("/api", accounts::routes().merge(listings::routes()))
        .layer(DefaultBodyLimit::max(6 * 1024 * 1024))
        .with_state(state);
    let address = env::var("API_ADDR").unwrap_or_else(|_| "127.0.0.1:3001".into());
    let listener = tokio::net::TcpListener::bind(&address).await?;
    println!("Travaasa API listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}

async fn health(State(state): State<App>) -> ApiResult<StatusCode> {
    state
        .db
        .run_command(doc! { "ping": 1 })
        .await
        .map_err(|error| {
            eprintln!("{error}");
            ApiError(StatusCode::SERVICE_UNAVAILABLE, "Database unavailable.")
        })?;
    Ok(StatusCode::NO_CONTENT)
}
