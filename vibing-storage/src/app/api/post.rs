use crate::{
    app::fetch::fetch_metadata_from,
    database::{
        core::pool::VibingPool,
        entities::track::{TrackFull, TrackMetadata},
    },
};
use axum::{Json, extract::State, http::StatusCode};
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn handle_upload_request(
    State(pool): State<Arc<RwLock<VibingPool>>>,
    Json(mut metadata): Json<TrackMetadata>,
) -> Result<StatusCode, StatusCode> {
    let default_metadata = match fetch_metadata_from(&metadata.path) {
        Ok(metadata) => metadata,
        Err(_) => {
            return Err(StatusCode::NOT_FOUND);
        }
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
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
