use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::database::{
    core::pool::VibingPool,
    entities::track::{TrackFullPatch, TrackFilter, TrackFull},
};

pub async fn get_root() -> String {
    "hello viber!".to_string()
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct ResponseVibe {
    pub group_name: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct ResponseTrack {
    pub id: i32,
    pub path: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub genre: Option<String>,
    pub duration: Option<i32>,
    pub vibes: Vec<ResponseVibe>,
    pub average_rating: f64,
    pub download_count: i32,
}

impl Into<ResponseTrack> for TrackFull {
    fn into(self) -> ResponseTrack {
        let mut vibes = Vec::new();
        for vibe in self.vibes {
            vibes.push(
                ResponseVibe { group_name: vibe.group_name, name: vibe.name }
            );
        }

        let average_rating = if self.vote_count != 0 {
            self.total_rating as f64 / self.vote_count as f64
        } else {
            0.00
        };

        ResponseTrack {
            id: self.track.id,
            path: self.track.path,
            title: self.track.title,
            author: self.track.author,
            genre: self.track.genre,
            duration: self.track.duration,
            vibes,
            average_rating,
            download_count: self.download_count
        }
    }
}

pub async fn get_tracks_by_filter(
    State(pool): State<Arc<RwLock<VibingPool>>>,
    Query(filter): Query<TrackFilter>
) -> (StatusCode, Json<Vec<ResponseTrack>>) {
    let tracks = TrackFull::get_by_filter(filter, pool).await
        .expect("cannot get filtered tracks");
    let mut response_tracks = Vec::new();
    for track in tracks {
        response_tracks.push(track.into());
    }

    (
        StatusCode::OK,
        Json(response_tracks)
    )
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct DownloadTrack {
    id: i32,
}

pub async fn get_download_path_by_id(
    State(pool): State<Arc<RwLock<VibingPool>>>,
    Query(target_track): Query<DownloadTrack>,
) -> (StatusCode, String) {
    if let Ok(track_full) = TrackFull::get_by_id(target_track.id, pool.clone()).await {
        let patch = TrackFullPatch {
            new_download: true,
            ..Default::default()
        };
        let track_full = match track_full.apply_patch(patch, pool).await {
            Ok(track_full) => track_full,
            Err(_) => { return (StatusCode::BAD_REQUEST, String::from("cannot apply patch")); }
        };
            
        (StatusCode::OK, track_full.track.path)
    } else {
        (StatusCode::NOT_FOUND, String::from("id not found"))
    }
}