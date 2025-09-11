use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::sync::RwLock;
use tokio_util::io::ReaderStream;

use crate::database::{
    core::pool::VibingPool,
    entities::track::{FullTrackPatch, TrackFilter, TrackFull},
};

pub async fn get_root() -> String {
    "hello viber!".to_string()
}

pub async fn get_tracks_by_filter(
    State(pool): State<Arc<RwLock<VibingPool>>>,
    Query(filter): Query<TrackFilter>
) -> (StatusCode, Json<Vec<TrackFull>>) {
    (
        StatusCode::OK,
        Json(TrackFull::get_by_filter(filter, pool).await.expect("cannot get filtered tracks"))
    )
}

fn path_to_mime_type(path: &str) -> &'static str {
    let extension = std::path::Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    match extension.to_lowercase().as_str() {
        "mp3" => "audio/mpeg",
        "flac" => "audio/flac",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        "m4a" => "audio/mp4",
        _ => "application/octet-stream",
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct DownloadTrack {
    id: i32,
}

pub async fn download_track_by_id(
    State(pool): State<Arc<RwLock<VibingPool>>>,
    Query(target_track): Query<DownloadTrack>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let track_full = TrackFull::get_by_id(target_track.id, pool.clone()).await.map_err(|_|
        (StatusCode::NOT_FOUND, format!("Track with id {} not found in database", target_track.id))
    )?;

    let file_path_str = track_full.track.path.clone();
    let file = match File::open(&file_path_str).await {
        Ok(file) => file,
        Err(_) => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("File not found on disk at path: {}", file_path_str),
            ));
        }
    };

    tokio::spawn(async move {
        let patch = FullTrackPatch {
            new_download: true,
            ..Default::default()
        };
        if let Err(e) = track_full.apply_patch(patch, pool).await {
            eprintln!("Failed to increment download count for track {}: {:?}", target_track.id, e);
        }
    });

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let filename = std::path::Path::new(&file_path_str)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("track")
        .to_string();

    let mime_type = path_to_mime_type(&filename);

    let headers = [
        (header::CONTENT_TYPE, mime_type.to_string()),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        ),
    ];

    Ok((headers, body))
}