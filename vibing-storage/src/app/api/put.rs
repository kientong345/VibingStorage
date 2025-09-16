use std::sync::Arc;
use axum::{
    extract::State,
    http::StatusCode,
    Json
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use crate::{
    database::{
        core::pool::VibingPool,
        entities::track::{TrackFull, TrackFullPatch}
    }
};

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct UserVote {
    track_id: i32,
    rating: u8,
}

pub async fn store_vote(
    State(pool): State<Arc<RwLock<VibingPool>>>,
    vote: Json<UserVote>
) -> Result<StatusCode, StatusCode> {
    let track = match TrackFull::get_by_id(vote.track_id, pool.clone()).await {
        Ok(track) => track,
        Err(_) => { return Err(StatusCode::NOT_FOUND); }
    };

    let patch = TrackFullPatch {
        new_vote: Some(vote.rating),
        ..Default::default()
    };

    match track.apply_patch(patch, pool).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::BAD_REQUEST)
    }
}