use crate::{
    app::services::download::DownloadableFile,
    database::{
        core::pool::VibingPool,
        entities::{
            Paginate,
            track::{TrackFilter, TrackFull, TrackFullPatch, TrackPaginationParams},
        },
    },
};
use axum::{
    Json,
    body::Body,
    extract::{Query, State},
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_util::io::ReaderStream;

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

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct PageFilterQuery {
    pub pattern: Option<String>,
    pub author: Option<String>,
    pub vibes: Option<Vec<String>>,
    pub limit: Option<i32>,
    pub order_by: Option<String>,
    pub page: i32,
    pub size: i32,
}

pub async fn get_filtered_page(
    State(pool): State<Arc<RwLock<VibingPool>>>,
    Query(filter): Query<PageFilterQuery>,
) -> Result<(StatusCode, Json<Vec<ResponseTrack>>), StatusCode> {
    let page = match TrackFull::page(&filter.into(), pool).await {
        Ok(page) => page,
        Err(_) => {
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    let mut response_tracks = Vec::new();
    for track in page.items {
        response_tracks.push(track.into());
    }

    Ok((StatusCode::OK, Json(response_tracks)))
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct DownloadQuery {
    pub track_id: i32,
}

pub async fn handle_download_request(
    State(pool): State<Arc<RwLock<VibingPool>>>,
    Query(target_track): Query<DownloadQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let track_full = match TrackFull::get_by_id(target_track.track_id, pool.clone()).await {
        Ok(track_full) => track_full,
        Err(_) => {
            return Err(StatusCode::NOT_FOUND);
        }
    };

    let path = &track_full.track.path;

    let downloadable_file = match DownloadableFile::get_from(&path).await {
        Ok(file) => file,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let body = Body::from_stream(ReaderStream::new(downloadable_file.file));

    let response = match Response::builder()
        .status(200)
        .header(header::CONTENT_TYPE, &downloadable_file.content_type)
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", downloadable_file.name),
        )
        .body(body)
    {
        Ok(response) => response,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let patch = TrackFullPatch {
        new_download: true,
        ..Default::default()
    };

    if track_full.apply_patch(patch, pool).await.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(response)
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct MusicStreamQuery {
    pub track_id: i32,
    pub start_at: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct ResponseMusicStream {
    pub track_id: i32,
    pub is_playing: bool,
    pub ellapsed_time: i32,
}

pub async fn handle_stream_request(
    State(pool): State<Arc<RwLock<VibingPool>>>,
    Query(target_track): Query<MusicStreamQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let track_full = match TrackFull::get_by_id(target_track.track_id, pool.clone()).await {
        Ok(track_full) => track_full,
        Err(_) => {
            return Err(StatusCode::NOT_FOUND);
        }
    };

    let path = &track_full.track.path;

    let downloadable_file = match DownloadableFile::get_from(&path).await {
        Ok(file) => file,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let body = Body::from_stream(ReaderStream::new(downloadable_file.file));

    let response = match Response::builder()
        .status(200)
        .header(header::CONTENT_TYPE, &downloadable_file.content_type)
        .header(header::CACHE_CONTROL, "no-cache")
        .body(body)
    {
        Ok(response) => response,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(response)
}

impl Into<ResponseTrack> for TrackFull {
    fn into(self) -> ResponseTrack {
        let mut vibes = Vec::new();
        for vibe in self.vibes {
            vibes.push(ResponseVibe {
                group_name: vibe.group_name,
                name: vibe.name,
            });
        }

        let average_rating = if self.track.vote_count != 0 {
            self.track.total_rating as f64 / self.track.vote_count as f64
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
            download_count: self.track.download_count,
        }
    }
}

impl Into<TrackPaginationParams> for PageFilterQuery {
    fn into(self) -> TrackPaginationParams {
        TrackPaginationParams {
            page_num: self.page,
            page_size: self.size,
            filter: TrackFilter {
                pattern: self.pattern,
                author: self.author,
                vibes: self.vibes,
                limit: self.limit,
                order_by: self.order_by,
            },
        }
    }
}
