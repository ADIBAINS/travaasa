use std::{collections::HashMap, env};

use axum::{
    Json, Router,
    body::Bytes,
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post},
};
use futures_util::TryStreamExt;
use mongodb::{
    Database, IndexModel,
    bson::{DateTime, Document, Regex, doc, oid::ObjectId},
};
use serde::{Deserialize, Serialize};

use crate::{ApiError, ApiResult, App, accounts::User};

const CATEGORIES: &[&str] = &["Beach", "Mountains", "Countryside", "City"];
const PAGE_SIZE: u64 = 12;

#[derive(Clone)]
pub struct Cloud {
    name: String,
    key: String,
    secret: String,
}

impl Cloud {
    pub fn from_env() -> Result<Option<Self>, &'static str> {
        match (
            env::var("CLOUD_NAME"),
            env::var("CLOUD_API_KEY"),
            env::var("CLOUD_API_SECRET"),
        ) {
            (Ok(name), Ok(key), Ok(secret))
                if !name.is_empty() && !key.is_empty() && !secret.is_empty() =>
            {
                Ok(Some(Self { name, key, secret }))
            }
            (Err(_), Err(_), Err(_)) => Ok(None),
            _ => Err("Configure all three Cloudinary variables or leave all three unset."),
        }
    }
}

#[derive(Deserialize)]
struct UploadedImage {
    secure_url: String,
    public_id: String,
}

#[derive(Deserialize)]
struct Image {
    url: Option<String>,
}

#[derive(Deserialize)]
struct Geometry {
    coordinates: Option<Vec<f64>>,
}

#[derive(Deserialize)]
struct StoredListing {
    #[serde(rename = "_id")]
    id: ObjectId,
    title: String,
    description: String,
    price: f64,
    location: String,
    country: String,
    category: Option<String>,
    image: Option<Image>,
    owner: Option<ObjectId>,
    geometry: Option<Geometry>,
    #[serde(default)]
    review: Vec<ObjectId>,
    #[serde(default)]
    reviews: Vec<StoredReview>,
}

#[derive(Deserialize)]
struct StoredReview {
    #[serde(rename = "_id")]
    id: ObjectId,
    comment: Option<String>,
    rating: Option<f64>,
    author: Option<ObjectId>,
    #[serde(rename = "createdAt")]
    created_at: Option<DateTime>,
}

#[derive(Serialize)]
struct Stay {
    id: String,
    title: String,
    location: String,
    country: String,
    category: Option<String>,
    price: f64,
    image: Option<String>,
}

impl StoredListing {
    fn stay(&self) -> Stay {
        Stay {
            id: self.id.to_hex(),
            title: self.title.clone(),
            location: self.location.clone(),
            country: self.country.clone(),
            category: self.category.clone(),
            price: self.price,
            image: self
                .image
                .as_ref()
                .and_then(|image| image.url.clone())
                .filter(|url| {
                    reqwest::Url::parse(url)
                        .is_ok_and(|url| matches!(url.scheme(), "https" | "http"))
                }),
        }
    }
}

#[derive(Serialize)]
struct Review {
    id: String,
    author: Option<User>,
    rating: f64,
    comment: String,
}

#[derive(Serialize)]
struct Detail {
    #[serde(flatten)]
    stay: Stay,
    description: String,
    owner: Option<User>,
    coordinates: Option<[f64; 2]>,
    reviews: Vec<Review>,
}

#[derive(Deserialize, Default)]
struct Filters {
    #[serde(default)]
    q: String,
    #[serde(default)]
    category: String,
    min: Option<f64>,
    max: Option<f64>,
    #[serde(default)]
    sort: String,
    page: Option<u64>,
}

#[derive(Serialize)]
struct Catalog {
    stays: Vec<Stay>,
    count: u64,
    page: u64,
    pages: u64,
}

#[derive(Deserialize)]
struct StayInput {
    title: String,
    description: String,
    location: String,
    country: String,
    category: String,
    price: f64,
}

#[derive(Deserialize)]
struct ReviewInput {
    rating: i32,
    comment: String,
}

pub fn routes() -> Router<App> {
    Router::new()
        .route("/listings", get(catalog).post(create))
        .route("/listings/mine", get(host_catalog))
        .route("/listings/saved", get(saved_catalog))
        .route("/listings/saved/ids", get(saved_listing_ids))
        .route("/listings/{id}", get(detail).put(update).delete(remove))
        .route(
            "/listings/{id}/saved",
            get(saved_status).put(save).delete(unsave),
        )
        .route("/listings/{id}/reviews", post(create_review))
        .route("/listings/{id}/reviews/{review}", delete(remove_review))
}

pub async fn ensure_indexes(db: &Database) -> mongodb::error::Result<()> {
    db.collection::<StoredListing>("listings")
        .create_indexes([
            IndexModel::builder()
                .keys(doc! { "category": 1, "_id": -1 })
                .build(),
            IndexModel::builder()
                .keys(doc! { "owner": 1, "_id": -1 })
                .build(),
            IndexModel::builder()
                .keys(doc! { "price": 1, "_id": -1 })
                .build(),
            IndexModel::builder()
                .keys(doc! { "price": -1, "_id": -1 })
                .build(),
        ])
        .await?;
    db.collection::<Document>("users")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "savedListings": 1 })
                .build(),
        )
        .await?;
    Ok(())
}

fn parse_id(id: &str) -> ApiResult<ObjectId> {
    ObjectId::parse_str(id).map_err(|_| ApiError::missing())
}

async fn catalog(
    State(state): State<App>,
    Query(filters): Query<Filters>,
) -> ApiResult<Json<Catalog>> {
    find_catalog(&state, filters, doc! {}).await
}

#[derive(Deserialize)]
struct Saved {
    #[serde(rename = "savedListings", default)]
    ids: Vec<ObjectId>,
}

async fn saved_ids(state: &App, user: &User) -> ApiResult<Vec<ObjectId>> {
    let record = state
        .db
        .collection::<Saved>("users")
        .find_one(doc! { "_id": user.id })
        .projection(doc! { "savedListings": 1 })
        .await?;
    Ok(record.map(|record| record.ids).unwrap_or_default())
}

async fn saved_catalog(
    State(state): State<App>,
    user: User,
    Query(filters): Query<Filters>,
) -> ApiResult<Json<Catalog>> {
    let ids = saved_ids(&state, &user).await?;
    find_catalog(&state, filters, doc! { "_id": { "$in": ids } }).await
}

async fn saved_listing_ids(State(state): State<App>, user: User) -> ApiResult<Json<Vec<String>>> {
    let ids = saved_ids(&state, &user).await?;
    Ok(Json(ids.into_iter().map(ObjectId::to_hex).collect()))
}

async fn host_catalog(
    State(state): State<App>,
    user: User,
    Query(filters): Query<Filters>,
) -> ApiResult<Json<Catalog>> {
    find_catalog(&state, filters, doc! { "owner": user.id }).await
}

async fn find_catalog(
    state: &App,
    filters: Filters,
    mut query: Document,
) -> ApiResult<Json<Catalog>> {
    let keyword = filters.q.trim();
    if keyword.chars().count() > 100 {
        return Err(ApiError::bad("Search up to 100 characters."));
    }
    if !keyword.is_empty() {
        let keyword = Regex {
            pattern: regex::escape(keyword),
            options: "i".into(),
        };
        query.insert(
            "$or",
            [
                doc! { "title": &keyword },
                doc! { "location": &keyword },
                doc! { "country": &keyword },
            ]
            .to_vec(),
        );
    }
    if !filters.category.is_empty() {
        if !CATEGORIES.contains(&filters.category.as_str()) {
            return Err(ApiError::bad("Choose a valid stay category."));
        }
        query.insert("category", filters.category);
    }
    let mut price = doc! {};
    if let Some(min) = filters.min {
        if !min.is_finite() || min < 0.0 {
            return Err(ApiError::bad("Enter a valid minimum price."));
        }
        price.insert("$gte", min);
    }
    if let Some(max) = filters.max {
        if !max.is_finite() || max < filters.min.unwrap_or(0.0) {
            return Err(ApiError::bad(
                "Maximum price must be at least the minimum price.",
            ));
        }
        price.insert("$lte", max);
    }
    if !price.is_empty() {
        query.insert("price", price);
    }
    let order = match filters.sort.as_str() {
        "" | "newest" => doc! { "_id": -1 },
        "price-low" => doc! { "price": 1, "_id": -1 },
        "price-high" => doc! { "price": -1, "_id": -1 },
        _ => return Err(ApiError::bad("Choose a valid sort order.")),
    };
    let listings = state.db.collection::<StoredListing>("listings");
    let count = listings.count_documents(query.clone()).await?;
    let pages = count.div_ceil(PAGE_SIZE).max(1);
    let page = filters.page.unwrap_or(1).clamp(1, pages);
    let items = listings
        .find(query)
        .sort(order)
        .skip((page - 1) * PAGE_SIZE)
        .limit(PAGE_SIZE as i64)
        .projection(doc! { "review": 0, "reviews": 0 })
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    Ok(Json(Catalog {
        stays: items.iter().map(StoredListing::stay).collect(),
        count,
        page,
        pages,
    }))
}

async fn detail(State(state): State<App>, Path(id): Path<String>) -> ApiResult<Json<Detail>> {
    let listing = state
        .db
        .collection::<StoredListing>("listings")
        .find_one(doc! { "_id": parse_id(&id)? })
        .await?
        .ok_or_else(ApiError::missing)?;
    let legacy = state
        .db
        .collection::<StoredReview>("reviews")
        .find(doc! { "_id": { "$in": &listing.review } })
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    let mut reviews = listing
        .reviews
        .iter()
        .chain(legacy.iter())
        .collect::<Vec<_>>();
    reviews.sort_by_key(|review| std::cmp::Reverse(review.created_at));
    let authors: Vec<_> = listing
        .owner
        .into_iter()
        .chain(reviews.iter().filter_map(|review| review.author))
        .collect();
    let users = state
        .db
        .collection::<User>("users")
        .find(doc! { "_id": { "$in": authors } })
        .projection(doc! { "username": 1 })
        .await?
        .try_collect::<Vec<_>>()
        .await?;
    let users: HashMap<_, _> = users.into_iter().map(|user| (user.id, user)).collect();
    let reviews = reviews
        .into_iter()
        .filter_map(|review| {
            Some(Review {
                id: review.id.to_hex(),
                author: review.author.and_then(|id| users.get(&id).cloned()),
                rating: review.rating?,
                comment: review.comment.clone().unwrap_or_default(),
            })
        })
        .collect();
    let coordinates = listing
        .geometry
        .as_ref()
        .and_then(|geometry| geometry.coordinates.as_ref())
        .and_then(|value| <[f64; 2]>::try_from(value.as_slice()).ok())
        .filter(|[lon, lat]| lon.abs() <= 180.0 && lat.abs() <= 90.0);
    Ok(Json(Detail {
        stay: listing.stay(),
        description: listing.description,
        owner: listing.owner.and_then(|id| users.get(&id).cloned()),
        coordinates,
        reviews,
    }))
}

fn required_text(value: &str, max: usize, error: &'static str) -> ApiResult<String> {
    let value = value.trim();
    if value.is_empty() || value.chars().count() > max {
        return Err(ApiError::bad(error));
    }
    Ok(value.to_owned())
}

async fn read_stay(mut form: Multipart) -> ApiResult<(Document, Option<Bytes>)> {
    let malformed = |_| ApiError::bad("Unable to read the listing form.");
    let (mut input, mut image) = (None, None);
    while let Some(field) = form.next_field().await.map_err(malformed)? {
        match field.name() {
            Some("stay") if input.is_none() => {
                let text = field.text().await.map_err(malformed)?;
                input = Some(serde_json::from_str::<StayInput>(&text).map_err(|_| {
                    ApiError::bad("Enter valid listing details and a nightly price.")
                })?);
            }
            Some("image") if image.is_none() => {
                let file = field.bytes().await.map_err(malformed)?;
                if file.len() > 5 * 1024 * 1024 {
                    return Err(ApiError::bad("Choose an image under 5 MB."));
                }
                if !file.is_empty() {
                    if !file.starts_with(b"\xff\xd8\xff") && !file.starts_with(b"\x89PNG\r\n\x1a\n")
                    {
                        return Err(ApiError::bad("Choose a JPEG or PNG image."));
                    }
                    image = Some(file);
                }
            }
            _ => return Err(ApiError::bad("Unexpected listing form field.")),
        }
    }
    let input = input.ok_or_else(|| ApiError::bad("Enter the listing details."))?;
    if !input.price.is_finite() || !(0.0..=1_000_000_000.0).contains(&input.price) {
        return Err(ApiError::bad(
            "Enter a nightly price from ₹0 to ₹1,000,000,000.",
        ));
    }
    if !input.category.is_empty() && !CATEGORIES.contains(&input.category.as_str()) {
        return Err(ApiError::bad("Choose a valid stay category."));
    }
    Ok((
        doc! {
            "title": required_text(&input.title, 150, "Enter a title up to 150 characters.")?,
            "description": required_text(&input.description, 5000, "Enter a description up to 5,000 characters.")?,
            "location": required_text(&input.location, 150, "Enter a location up to 150 characters.")?,
            "country": required_text(&input.country, 100, "Enter a country up to 100 characters.")?,
            "price": input.price,
            "category": (!input.category.is_empty()).then_some(input.category),
        },
        image,
    ))
}

async fn upload_image(state: &App, file: Bytes) -> ApiResult<Document> {
    let cloud = state.cloud.as_ref().ok_or(ApiError(
        StatusCode::SERVICE_UNAVAILABLE,
        "Image uploads are currently unavailable.",
    ))?;
    let filename = if file.starts_with(b"\xff\xd8\xff") {
        "stay.jpg"
    } else {
        "stay.png"
    };
    let form = reqwest::multipart::Form::new()
        .text("folder", "travasaa_Dev")
        .part(
            "file",
            reqwest::multipart::Part::bytes(file.to_vec()).file_name(filename),
        );
    let response = async {
        state
            .http
            .post(format!(
                "https://api.cloudinary.com/v1_1/{}/image/upload",
                cloud.name
            ))
            .basic_auth(&cloud.key, Some(&cloud.secret))
            .multipart(form)
            .send()
            .await?
            .error_for_status()?
            .json::<UploadedImage>()
            .await
    }
    .await
    .map_err(|_| {
        ApiError(
            StatusCode::BAD_GATEWAY,
            "Image storage is temporarily unavailable.",
        )
    })?;
    Ok(doc! { "url": response.secure_url, "filename": response.public_id })
}

async fn geocode(state: &App, listing: &Document) -> Option<Document> {
    let token = state.mapbox_token.as_ref()?;
    let location = listing.get_str("location").ok()?;
    let country = listing.get_str("country").ok()?;
    let query = format!("{location}, {country}");
    let response = state
        .http
        .get("https://api.mapbox.com/search/geocode/v6/forward")
        .query(&[
            ("q", query.as_str()),
            ("access_token", token),
            ("limit", "1"),
            ("permanent", "true"),
        ])
        .send()
        .await
        .ok()?
        .error_for_status()
        .ok()?
        .json::<serde_json::Value>()
        .await
        .ok()?;
    let coords = response
        .pointer("/features/0/geometry/coordinates")?
        .as_array()?;
    let [lon, lat] = coords.as_slice() else {
        return None;
    };
    Some(doc! { "type": "Point", "coordinates": [lon.as_f64()?, lat.as_f64()?] })
}

#[derive(Serialize)]
struct Created {
    id: String,
}

async fn create(
    State(state): State<App>,
    user: User,
    form: Multipart,
) -> ApiResult<(StatusCode, Json<Created>)> {
    let (mut listing, image) = read_stay(form).await?;
    if let Some(geometry) = geocode(&state, &listing).await {
        listing.insert("geometry", geometry);
    }
    if let Some(image) = image {
        listing.insert("image", upload_image(&state, image).await?);
    }
    let id = ObjectId::new();
    listing.insert("_id", id);
    listing.insert("owner", user.id);
    state.db.collection("listings").insert_one(listing).await?;
    Ok((StatusCode::CREATED, Json(Created { id: id.to_hex() })))
}

fn forbidden() -> ApiError {
    ApiError(
        StatusCode::FORBIDDEN,
        "You do not have permission to make this change.",
    )
}

async fn update(
    State(state): State<App>,
    user: User,
    Path(id): Path<String>,
    form: Multipart,
) -> ApiResult<StatusCode> {
    let id = parse_id(&id)?;
    let collection = state.db.collection::<StoredListing>("listings");
    let current = collection
        .find_one(doc! { "_id": id })
        .await?
        .ok_or_else(ApiError::missing)?;
    if current.owner != Some(user.id) {
        return Err(forbidden());
    }
    let (mut listing, image) = read_stay(form).await?;
    let mut unset = doc! {};
    if listing.get_str("location").ok() != Some(&current.location)
        || listing.get_str("country").ok() != Some(&current.country)
    {
        match geocode(&state, &listing).await {
            Some(geometry) => {
                listing.insert("geometry", geometry);
            }
            None => {
                unset.insert("geometry", "");
            }
        }
    }
    if let Some(image) = image {
        listing.insert("image", upload_image(&state, image).await?);
    }
    let mut update = doc! { "$set": listing };
    if !unset.is_empty() {
        update.insert("$unset", unset);
    }
    let result = collection
        .update_one(
            doc! {
                "_id": id, "owner": user.id,
                "location": current.location, "country": current.country
            },
            update,
        )
        .await?;
    if result.matched_count == 0 {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "The listing changed while you were editing it. Reload and try again.",
        ));
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn remove(
    State(state): State<App>,
    user: User,
    Path(id): Path<String>,
) -> ApiResult<StatusCode> {
    let id = parse_id(&id)?;
    let listings = state.db.collection::<StoredListing>("listings");
    let removed = listings
        .find_one_and_delete(doc! { "_id": id, "owner": user.id })
        .await?;
    let Some(removed) = removed else {
        return Err(if listings.find_one(doc! { "_id": id }).await?.is_some() {
            forbidden()
        } else {
            ApiError::missing()
        });
    };
    if !removed.review.is_empty() {
        state
            .db
            .collection::<StoredReview>("reviews")
            .delete_many(doc! { "_id": { "$in": removed.review } })
            .await?;
    }
    state
        .db
        .collection::<Saved>("users")
        .update_many(
            doc! { "savedListings": id },
            doc! { "$pull": { "savedListings": id } },
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn saved_status(
    State(state): State<App>,
    user: User,
    Path(id): Path<String>,
) -> ApiResult<Json<bool>> {
    let id = parse_id(&id)?;
    Ok(Json(saved_ids(&state, &user).await?.contains(&id)))
}

async fn save(
    State(state): State<App>,
    user: User,
    Path(id): Path<String>,
) -> ApiResult<StatusCode> {
    let id = parse_id(&id)?;
    if state
        .db
        .collection::<Document>("listings")
        .find_one(doc! { "_id": id })
        .projection(doc! { "_id": 1 })
        .await?
        .is_none()
    {
        return Err(ApiError::missing());
    }
    state
        .db
        .collection::<Saved>("users")
        .update_one(
            doc! { "_id": user.id },
            doc! { "$addToSet": { "savedListings": id } },
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn unsave(
    State(state): State<App>,
    user: User,
    Path(id): Path<String>,
) -> ApiResult<StatusCode> {
    state
        .db
        .collection::<Saved>("users")
        .update_one(
            doc! { "_id": user.id },
            doc! { "$pull": { "savedListings": parse_id(&id)? } },
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn create_review(
    State(state): State<App>,
    user: User,
    Path(id): Path<String>,
    Json(review): Json<ReviewInput>,
) -> ApiResult<StatusCode> {
    let id = parse_id(&id)?;
    let comment = required_text(
        &review.comment,
        2500,
        "Enter a review up to 2,500 characters.",
    )?;
    if !(1..=5).contains(&review.rating) {
        return Err(ApiError::bad("Choose a rating from 1 to 5 stars."));
    }
    let result = state
        .db
        .collection::<StoredListing>("listings")
        .update_one(
            doc! { "_id": id },
            doc! { "$push": { "reviews": { "_id": ObjectId::new(), "author": user.id,
            "rating": review.rating, "comment": comment, "createdAt": DateTime::now() } } },
        )
        .await?;
    if result.matched_count == 0 {
        return Err(ApiError::missing());
    }
    Ok(StatusCode::CREATED)
}

async fn remove_review(
    State(state): State<App>,
    user: User,
    Path((id, review)): Path<(String, String)>,
) -> ApiResult<StatusCode> {
    let (id, review) = (parse_id(&id)?, parse_id(&review)?);
    let listings = state.db.collection::<StoredListing>("listings");
    let owned_review = doc! { "_id": review, "author": user.id };
    let embedded = listings
        .update_one(
            doc! { "_id": id, "reviews": { "$elemMatch": &owned_review } },
            doc! { "$pull": { "reviews": &owned_review } },
        )
        .await?;
    if embedded.modified_count > 0 {
        return Ok(StatusCode::NO_CONTENT);
    }

    let legacy_reviews = state.db.collection::<StoredReview>("reviews");
    if legacy_reviews
        .find_one(owned_review.clone())
        .await?
        .is_some()
    {
        let removed = listings
            .update_one(
                doc! { "_id": id, "review": review },
                doc! { "$pull": { "review": review } },
            )
            .await?;
        if removed.modified_count > 0 {
            legacy_reviews.delete_one(owned_review).await?;
            return Ok(StatusCode::NO_CONTENT);
        }
    }
    Err(ApiError(StatusCode::NOT_FOUND, "Review not found."))
}
