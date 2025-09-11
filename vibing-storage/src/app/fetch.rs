use std::{fs, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{app::error::Result, config::Configuration, database::{core::pool::VibingPool, entities::track::{sync_sample, SampleTrack, TrackMetadata}}};

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
struct SampleVibe {
    group: String,
    vibe: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
struct SampleTrackWithVibes {
    path: String,
    vibes: Vec<SampleVibe>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SampleRoot {
    tracks_with_vibes: Vec<SampleTrackWithVibes>
}

impl SampleRoot {
    pub fn fetch() -> SampleRoot {
        let sample_dir = Configuration::get().sample_dir
            .unwrap_or("".to_string());

        if sample_dir.is_empty() {
            return SampleRoot::default();
        }

        let header_path = format!("{}/vibing_header.json", sample_dir);

        let header_str = fs::read_to_string(header_path)
            .expect("cannot get sample header content");

        let mut sample: SampleRoot = serde_json::from_str(&header_str)
            .expect("cannot get sample header content");

        for track in &mut sample.tracks_with_vibes {
            track.path = format!("{}/{}", sample_dir, track.path);
        }

        sample
    }

    pub async fn sync(&self, pool: Arc<RwLock<VibingPool>>) -> Result<()> {
        let mut sample_tracks: Vec<SampleTrack> = Vec::new();
        for track in &self.tracks_with_vibes {
            let metadata = fetch_metadata_from(&track.path)
                .expect("cannot get metadata");

            let mut vibes = Vec::new();
            for vibe in &track.vibes {
                vibes.push(
                    (vibe.group.clone(), vibe.vibe.clone())
                );
            }

            sample_tracks.push(
                SampleTrack { metadata, vibes }
            );
        }

        sync_sample(sample_tracks, pool).await;

        Ok(())
    }
}



pub fn fetch_metadata_from(path: &str) -> Result<TrackMetadata> {
    let tag = audiotags::Tag::new().read_from_path(path)?;

    let duration = if let Some(dur) = tag.duration() {
        Some(dur as i32)
    } else {
        None
    };

    Ok(TrackMetadata {
        path: path.to_string(),
        title: tag.title().map(String::from),
        author: tag.artist().map(String::from),
        genre: tag.genre().map(String::from),
        duration,
    })
}