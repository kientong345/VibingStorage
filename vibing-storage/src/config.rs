use std::fs;

use serde::Deserialize;


pub fn database_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL is not set")
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Configuration {
    pub resource_dir: Option<String>,
    pub sample_dir: Option<String>,
    pub port: u16,
}

impl Configuration {
    pub fn get() -> Configuration {
        let content = fs::read_to_string("config.json")
            .expect("cannot get config data");

        serde_json::from_str(&content)
            .expect("cannot get config data")
    }
}
