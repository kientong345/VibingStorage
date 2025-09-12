use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use tokio::sync::RwLock;

use crate::{app::fetch::fetch_metadata_from, database::{core::pool::VibingPool, entities::track::TrackFull}};

pub async fn upload_track(
    State(pool): State<Arc<RwLock<VibingPool>>>,
    path: String
) -> StatusCode {
    let metadata = match fetch_metadata_from(&path) {
        Ok(metadata) => metadata,
        Err(_) => { return StatusCode::INTERNAL_SERVER_ERROR; }
    };
    match TrackFull::create_from(metadata, pool).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::BAD_REQUEST
    }
}