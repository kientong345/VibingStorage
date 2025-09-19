use crate::database::{core::pool::VibingPool, error::Result};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq, FromRow)]
pub struct Vibe {
    pub id: i32,
    pub name: String,
    pub group_name: String,
}

impl Vibe {
    pub async fn get_by_id(id: i32, pool: Arc<RwLock<VibingPool>>) -> Result<Vibe> {
        Ok(sqlx::query_as!(
            Vibe,
            "
            SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
            FROM vibes AS vb
            JOIN vibe_groups AS vg ON vb.vibe_group = vg.vibe_group_id
            WHERE vb.vibe_id = $1
            ",
            id
        )
        .fetch_one(pool.read().await.get_inner())
        .await?)
    }

    pub async fn get_by_name(name: &str, pool: Arc<RwLock<VibingPool>>) -> Result<Vibe> {
        Ok(sqlx::query_as!(
            Vibe,
            "
            SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
            FROM vibes AS vb
            JOIN vibe_groups AS vg ON vb.vibe_group = vg.vibe_group_id
            WHERE vb.name = $1
            ",
            name
        )
        .fetch_one(pool.read().await.get_inner())
        .await?)
    }

    pub async fn get_all(pool: Arc<RwLock<VibingPool>>) -> Result<Vec<Vibe>> {
        Ok(sqlx::query_as!(
            Vibe,
            "
            SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
            FROM vibes AS vb
            JOIN vibe_groups AS vg ON vb.vibe_group = vg.vibe_group_id
            "
        )
        .fetch_all(pool.read().await.get_inner())
        .await?)
    }

    pub async fn count(pool: Arc<RwLock<VibingPool>>) -> Result<i64> {
        Ok(sqlx::query!(
            "
            SELECT COUNT(*) AS vibes_count
            FROM vibes
            "
        )
        .fetch_one(pool.read().await.get_inner())
        .await?
        .vibes_count
        .unwrap_or(-1))
    }
}
