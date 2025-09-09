use serde::Deserialize;

use crate::database::{core::pool::VibingPool, entities::track::Track, error::Result};


#[derive(Debug, Deserialize, Clone, Default)]
pub struct Vibe {
    pub id: i32,
    pub name: String,
    pub group_name: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct VibeFull {
    pub vibe: Vibe,
    pub tracks: Vec<Track>,
}

impl Vibe {
    pub async fn get_by_id(id: i32, pool: &VibingPool) -> Result<Vibe> {
        Ok(sqlx::query_as!(Vibe,
            "
            SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
            FROM vibes AS vb
            JOIN vibe_groups AS vg ON vb.vibe_group = vg.vibe_group_id
            WHERE vb.vibe_id = $1
            ", id
        ).fetch_one(pool.get_inner()).await?)
    }

    pub async fn get_by_name(name: &str, pool: &VibingPool) -> Result<Vibe> {
        Ok(sqlx::query_as!(Vibe,
            "
            SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
            FROM vibes AS vb
            JOIN vibe_groups AS vg ON vb.vibe_group = vg.vibe_group_id
            WHERE vb.name = $1
            ", name
        ).fetch_one(pool.get_inner()).await?)
    }
}

impl VibeFull {
    pub async fn get_by_id(id: i32, pool: &VibingPool) -> Result<VibeFull> {
        let vibe = Vibe::get_by_id(id, pool).await?;

        let tracks = sqlx::query_as!(Track,
            "
            SELECT tr.track_id AS id, tr.path AS path, tr.title AS title, tr.author AS author, tr.genre AS genre, tr.duration AS duration
            FROM tracks_with_vibes AS twv
            JOIN tracks AS tr ON twv.track = tr.track_id
            WHERE twv.vibe = $1
            ", id
        ).fetch_all(pool.get_inner()).await?;

        Ok(VibeFull { vibe, tracks })
    }

    pub async fn get_by_name(name: &str, pool: &VibingPool) -> Result<VibeFull> {
        let vibe = Vibe::get_by_name(name, pool).await?;

        let tracks = sqlx::query_as!(Track,
            "
            SELECT tr.track_id AS id, tr.path AS path, tr.title AS title, tr.author AS author, tr.genre AS genre, tr.duration AS duration
            FROM tracks_with_vibes AS twv
            JOIN tracks AS tr ON twv.track = tr.track_id
            JOIN vibes AS vb ON twv.vibe = vb.vibe_id
            WHERE vb.name = $1
            ", name
        ).fetch_all(pool.get_inner()).await?;

        Ok(VibeFull { vibe, tracks })
    }
}