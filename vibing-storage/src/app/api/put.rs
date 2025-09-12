use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::database::{core::pool::VibingPool, entities::track::{TrackFull, TrackFullPatch}};

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct UserVote {
    track_id: i32,
    rating: u8,
}

pub async fn store_vote(
    State(pool): State<Arc<RwLock<VibingPool>>>,
    vote: Json<UserVote>
) -> StatusCode {
    let track = match TrackFull::get_by_id(vote.track_id, pool.clone()).await {
        Ok(track) => track,
        Err(_) => { return StatusCode::NOT_FOUND; }
    };

    let patch = TrackFullPatch {
        new_vote: Some(vote.rating),
        ..Default::default()
    };

    match track.apply_patch(patch, pool).await {
        Ok(_) => { return StatusCode::OK; },
        Err(_) => { return StatusCode::INTERNAL_SERVER_ERROR; }
    }
}