use crate::database::{
    core::pool::VibingPool,
    entities::track::{TrackFull, TrackFullPatch},
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct TrackPatchQuery {
    pub id: i32,
    pub path: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub genre: Option<String>,
    pub duration: Option<i32>,
    pub rating: Option<u8>,
    pub add_vibes: Option<Vec<(String, String)>>,
    pub remove_vibes: Option<Vec<(String, String)>>,
}

pub async fn update_track(
    State(pool): State<Arc<RwLock<VibingPool>>>,
    Query(patch): Query<TrackPatchQuery>,
) -> Result<StatusCode, StatusCode> {
    let (track_id, track_patch) = patch.into();

    let track = match TrackFull::get_by_id(track_id, pool.clone()).await {
        Ok(track) => track,
        Err(_) => {
            return Err(StatusCode::NOT_FOUND);
        }
    };

    if track.apply_patch(track_patch, pool).await.is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(StatusCode::OK)
}

impl Into<(i32, TrackFullPatch)> for TrackPatchQuery {
    fn into(self) -> (i32, TrackFullPatch) {
        (
            self.id,
            TrackFullPatch {
                path: self.path,
                title: self.title,
                author: self.author,
                genre: self.genre,
                duration: self.duration,
                new_vote: self.rating,
                new_download: false,
                add_vibes: self.add_vibes,
                remove_vibes: self.remove_vibes,
            },
        )
    }
}
