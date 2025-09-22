use crate::{app::error::Result, database::entities::track::TrackMetadata};
use std::fs;

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

pub fn fetch_resource_from(path: &str) -> Result<Vec<TrackMetadata>> {
    let mut metadata_vec = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if let Some(extension) = path.extension() {
            if extension != "mp3" {
                continue;
            }
        }

        if let Some(path_str) = path.to_str() {
            if let Ok(metadata) = fetch_metadata_from(path_str) {
                metadata_vec.push(metadata);
            }
        }
    }

    Ok(metadata_vec)
}
