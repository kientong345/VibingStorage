use std::sync::Arc;

use axum::{extract::{Query, State}, http::StatusCode};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{database::{core::pool::VibingPool, entities::track::TrackFull}};


#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct DeleteTrack {
    id: Option<i32>,
    title: Option<String>,
}

pub async fn delete_track(
    State(pool): State<Arc<RwLock<VibingPool>>>,
    Query(target): Query<DeleteTrack>
) -> StatusCode {
    if let Some(id) = target.id {
        let track = match TrackFull::get_by_id(id, pool.clone()).await {
            Ok(track) => track,
            Err(_) => { return StatusCode::NOT_FOUND; }
        };

        match track.remove(pool).await {
            Ok(_) => { return StatusCode::OK; },
            Err(_) => { return StatusCode::INTERNAL_SERVER_ERROR; }
        }
    }

    if let Some(title) = target.title {
        let track = match TrackFull::get_by_title(&title, pool.clone()).await {
            Ok(track) => track,
            Err(_) => { return StatusCode::NOT_FOUND; }
        };

        match track.remove(pool).await {
            Ok(_) => { return StatusCode::OK; },
            Err(_) => { return StatusCode::INTERNAL_SERVER_ERROR; }
        }
    }

    StatusCode::BAD_REQUEST
}