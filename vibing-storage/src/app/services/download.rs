use crate::app::error::Result;
use std::path::Path;
use tokio::fs::File;

#[derive(Debug)]
pub struct DownloadableFile {
    pub name: String,
    pub content_type: String,
    pub file: File,
}

impl DownloadableFile {
    pub async fn get_from(path: &str) -> Result<Self> {
        let name = Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let content_type = match Path::new(path).extension().and_then(|ext| ext.to_str()) {
            Some("txt") => String::from("text/plain"),
            Some("pdf") => String::from("application/pdf"),
            Some("jpg") | Some("jpeg") => String::from("image/jpeg"),
            Some("png") => String::from("image/png"),
            Some("mp3") => String::from("audio/mpeg"),
            _ => String::from("application/octet-stream"), // Mặc định cho các file không xác định
        };

        let file = File::open(path).await?;

        Ok(Self {
            name,
            content_type,
            file,
        })
    }
}
