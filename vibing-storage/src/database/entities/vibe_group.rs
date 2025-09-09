use serde::Deserialize;

use crate::database::{core::pool::VibingPool, entities::vibe::Vibe, error::Result};

#[derive(Debug, Deserialize, Clone, Default)]
pub struct VibeGroup {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct VibeGroupFull {
    pub group: VibeGroup,
    pub vibes: Vec<Vibe>,
}

impl VibeGroup {
    pub async fn get_by_id(id: i32, pool: &VibingPool) -> Result<VibeGroup> {
        Ok(sqlx::query_as!(VibeGroup,
            "
            SELECT vibe_group_id AS id, name
            FROM vibe_groups
            WHERE vibe_group_id = $1
            ", id
        ).fetch_one(pool.get_inner()).await?)
    }

    pub async fn get_by_name(name: &str, pool: &VibingPool) -> Result<VibeGroup> {
        Ok(sqlx::query_as!(VibeGroup,
            "
            SELECT vibe_group_id AS id, name
            FROM vibe_groups
            WHERE name = $1
            ", name
        ).fetch_one(pool.get_inner()).await?)
    }

    pub async fn get_all(pool: &VibingPool) -> Vec<VibeGroup> {
        sqlx::query_as!(VibeGroup,
            "
            SELECT vibe_group_id AS id, name
            FROM vibe_groups
            "
        ).fetch_all(pool.get_inner()).await
        .unwrap_or(Vec::new())
    }

    pub async fn count(pool: &VibingPool) -> Result<i64> {
        Ok(sqlx::query!(
            "
            SELECT COUNT(*) AS groups_count
            FROM vibe_groups
            "
        ).fetch_one(pool.get_inner()).await?
        .groups_count
        .unwrap_or(-1))
    }
}

impl VibeGroupFull {
    pub async fn get_by_id(id: i32, pool: &VibingPool) -> Result<VibeGroupFull> {
        let group = VibeGroup::get_by_id(id, pool).await?;

        let vibes = sqlx::query_as!(Vibe,
            "
            SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
            FROM vibe_groups AS vg
            JOIN vibes AS vb ON vg.vibe_group_id = vb.vibe_group
            WHERE vg.vibe_group_id = $1
            ", id
        ).fetch_all(pool.get_inner()).await?;

        Ok(VibeGroupFull { group, vibes })
    }

    pub async fn get_by_name(name: &str, pool: &VibingPool) -> Result<VibeGroupFull> {
        let group = VibeGroup::get_by_name(name, pool).await?;

        let vibes = sqlx::query_as!(Vibe,
            "
            SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
            FROM vibe_groups AS vg
            JOIN vibes AS vb ON vg.vibe_group_id = vb.vibe_group
            WHERE vg.name = $1
            ", name
        ).fetch_all(pool.get_inner()).await?;

        Ok(VibeGroupFull { group, vibes })
    }

    pub async fn get_all(pool: &VibingPool) -> Vec<VibeGroupFull> {
        let groups = VibeGroup::get_all(pool).await;

        let mut full_groups = Vec::new();

        for group in groups {
            let group_id = group.id;
            let vibes = sqlx::query_as!(Vibe,
                "
                SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
                FROM vibe_groups AS vg
                JOIN vibes AS vb ON vg.vibe_group_id = vb.vibe_group
                WHERE vg.vibe_group_id = $1
                ", group_id
            ).fetch_all(pool.get_inner()).await
            .unwrap_or(Vec::new());

            full_groups.push(
                VibeGroupFull { group, vibes }
            );
        }

        full_groups
    }

    pub async fn count(pool: &VibingPool) -> Result<i64> {
        VibeGroup::count(pool).await
    }
}