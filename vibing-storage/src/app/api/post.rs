use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use tokio::sync::RwLock;

use crate::{app::fetch::fetch_metadata_from, database::{core::pool::VibingPool, entities::track::{TrackFull, TrackMetadata}}};

pub async fn upload_track(
    State(pool): State<Arc<RwLock<VibingPool>>>,
    Json(mut metadata): Json<TrackMetadata>
) -> StatusCode {
    let default_metadata = match fetch_metadata_from(&metadata.path) {
        Ok(metadata) => metadata,
        Err(_) => { return StatusCode::INTERNAL_SERVER_ERROR; }
    };

    if metadata.title.is_none() && default_metadata.title.is_some() {
        metadata.title = default_metadata.title;
    }

    if metadata.author.is_none() && default_metadata.author.is_some() {
        metadata.author = default_metadata.author;
    }

    if metadata.genre.is_none() && default_metadata.genre.is_some() {
        metadata.genre = default_metadata.genre;
    }

    if metadata.duration.is_none() && default_metadata.duration.is_some() {
        metadata.duration = default_metadata.duration;
    }

    match TrackFull::create_from(metadata, pool).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::BAD_REQUEST
    }
}