use crate::database::{core::pool::VibingPool, error::Result};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq, FromRow)]
pub struct Vibe {
    pub id: i32,
    pub name: String,
    pub group_name: Option<String>,
}

impl Vibe {
    pub async fn get_by_id(id: i32, pool: &VibingPool) -> Result<Vibe> {
        Ok(sqlx::query_as!(
            Vibe,
            "
            SELECT vibe_id AS id, name, group_name
            FROM vibes
            WHERE vibe_id = $1
            ",
            id
        )
        .fetch_one(pool.get_inner())
        .await?)
    }

    pub async fn get_by_name(name: &str, pool: &VibingPool) -> Result<Vibe> {
        Ok(sqlx::query_as!(
            Vibe,
            "
            SELECT vibe_id AS id, name, group_name
            FROM vibes
            WHERE name = $1
            ",
            name
        )
        .fetch_one(pool.get_inner())
        .await?)
    }

    pub async fn get_all(pool: &VibingPool) -> Result<Vec<Vibe>> {
        Ok(sqlx::query_as!(
            Vibe,
            "
            SELECT vibe_id AS id, name, group_name
            FROM vibes
            "
        )
        .fetch_all(pool.get_inner())
        .await?)
    }

    pub async fn get_by_track_id(id: i32, pool: &VibingPool) -> Result<Vec<Vibe>> {
        Ok(sqlx::query_as!(
            Vibe,
            "
            SELECT vb.vibe_id AS id, vb.name, vb.group_name
            FROM vibes AS vb
            JOIN tracks_with_vibes AS twv ON vb.vibe_id = twv.vibe
            WHERE twv.track = $1
            ",
            id
        )
        .fetch_all(pool.get_inner())
        .await?)
    }

    pub async fn get_by_track_ids(
        ids: &[i32],
        pool: &VibingPool,
    ) -> Result<HashMap<i32, Vec<Vibe>>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                twv.track AS track_id,
                vb.vibe_id AS id,
                vb.name AS name,
                vb.group_name
            FROM vibes AS vb
            JOIN tracks_with_vibes AS twv ON vb.vibe_id = twv.vibe
            WHERE twv.track = ANY($1)
            "#,
            ids
        )
        .fetch_all(pool.get_inner())
        .await?;

        let mut vibes_map: HashMap<i32, Vec<Vibe>> = HashMap::new();
        for row in rows {
            let vibe = Vibe {
                id: row.id,
                name: row.name,
                group_name: row.group_name,
            };
            vibes_map.entry(row.track_id).or_default().push(vibe);
        }

        Ok(vibes_map)
    }

    pub async fn count(pool: &VibingPool) -> Result<i64> {
        Ok(sqlx::query!(
            "
            SELECT COUNT(*) AS vibes_count
            FROM vibes
            "
        )
        .fetch_one(pool.get_inner())
        .await?
        .vibes_count
        .unwrap_or(-1))
    }
}
