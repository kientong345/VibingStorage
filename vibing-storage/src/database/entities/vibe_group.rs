use crate::database::{core::pool::VibingPool, entities::vibe::Vibe, error::Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct VibeGroup {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct VibeGroupFull {
    pub group: VibeGroup,
    pub vibes: Vec<Vibe>,
}

impl VibeGroupFull {
    pub async fn get_by_id(id: i32, pool: Arc<RwLock<VibingPool>>) -> Result<VibeGroupFull> {
        let pool_guard = pool.read().await;

        let group = sqlx::query_as!(
            VibeGroup,
            "
            SELECT vibe_group_id AS id, name
            FROM vibe_groups
            WHERE vibe_group_id = $1
            ",
            id
        )
        .fetch_one(pool_guard.get_inner())
        .await?;

        let vibes = sqlx::query_as!(
            Vibe,
            "
            SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
            FROM vibe_groups AS vg
            JOIN vibes AS vb ON vg.vibe_group_id = vb.vibe_group
            WHERE vg.vibe_group_id = $1
            ",
            id
        )
        .fetch_all(pool_guard.get_inner())
        .await?;

        Ok(VibeGroupFull { group, vibes })
    }

    pub async fn get_by_name(name: &str, pool: Arc<RwLock<VibingPool>>) -> Result<VibeGroupFull> {
        let pool_guard = pool.read().await;

        let group = sqlx::query_as!(
            VibeGroup,
            "
            SELECT vibe_group_id AS id, name
            FROM vibe_groups
            WHERE name = $1
            ",
            name
        )
        .fetch_one(pool_guard.get_inner())
        .await?;

        let vibes = sqlx::query_as!(
            Vibe,
            "
            SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
            FROM vibe_groups AS vg
            JOIN vibes AS vb ON vg.vibe_group_id = vb.vibe_group
            WHERE vg.name = $1
            ",
            name
        )
        .fetch_all(pool_guard.get_inner())
        .await?;

        Ok(VibeGroupFull { group, vibes })
    }

    pub async fn get_all(pool: Arc<RwLock<VibingPool>>) -> Result<Vec<VibeGroupFull>> {
        let pool_guard = pool.read().await;

        let groups = sqlx::query_as!(
            VibeGroup,
            "
            SELECT vibe_group_id AS id, name
            FROM vibe_groups
            "
        )
        .fetch_all(pool_guard.get_inner())
        .await?;

        if groups.is_empty() {
            return Ok(Vec::new());
        }

        let group_ids: Vec<i32> = groups.iter().map(|group| group.id).collect();

        let vibes = sqlx::query_as!(
            Vibe,
            r#"
            SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
            FROM vibe_groups AS vg
            JOIN vibes AS vb ON vg.vibe_group_id = vb.vibe_group
            WHERE vg.vibe_group_id = ANY($1)
            "#,
            &group_ids
        )
        .fetch_all(pool_guard.get_inner())
        .await?;

        let mut vibes_map: HashMap<i32, Vec<Vibe>> = HashMap::new();
        for vibe in vibes {
            vibes_map.entry(vibe.id).or_default().push(vibe);
        }

        let full_groups = groups
            .into_iter()
            .map(|group| {
                let vibes = vibes_map.remove(&group.id).unwrap_or_default();
                VibeGroupFull { group, vibes }
            })
            .collect();

        Ok(full_groups)
    }

    pub async fn count(pool: Arc<RwLock<VibingPool>>) -> Result<i64> {
        Ok(sqlx::query!(
            "
            SELECT COUNT(*) AS groups_count
            FROM vibe_groups
            "
        )
        .fetch_one(pool.read().await.get_inner())
        .await?
        .groups_count
        .unwrap_or(-1))
    }
}
