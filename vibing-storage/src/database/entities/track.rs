use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, QueryBuilder};
use tokio::sync::RwLock;

use crate::database::{core::pool::VibingPool, entities::vibe::Vibe, error::Result};

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq, FromRow)]
pub struct Track {
    pub id: i32,
    pub path: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub genre: Option<String>,
    pub duration: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct TrackPatch {
    pub path: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub genre: Option<String>,
    pub duration: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct TrackFull {
    pub track: Track,
    pub vibes: Vec<Vibe>,
    pub vote_count: i32,
    pub total_rating: i64,
    pub download_count: i32,
}

pub type GroupName = String;
pub type VibeName = String;

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct FullTrackPatch {
    pub track: Option<TrackPatch>,
    pub add_vibes: Vec<(GroupName, VibeName)>,
    pub remove_vibes: Vec<(GroupName, VibeName)>,
    pub new_vote: Option<u8>,
    pub new_download: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct TrackMetadata {
    pub path: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub genre: Option<String>,
    pub duration: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct TrackFilter {
    pub pattern: Option<String>,
    pub author: Option<String>,
    pub vibes: Option<Vec<String>>,
    pub limit: Option<i32>,
    pub order_by: Option<String>,
}

impl Track {
    pub async fn create_from(metadata: TrackMetadata, pool: Arc<RwLock<VibingPool>>) -> Result<Track> {
        Ok(sqlx::query_as!(Track,
            r#"
            INSERT INTO tracks (path, title, author, genre, duration)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING track_id AS id, path, title, author, genre, duration
            "#, metadata.path, metadata.title, metadata.author, metadata.genre, metadata.duration
        ).fetch_one(pool.read().await.get_inner()).await?)
    }

    pub async fn get_by_id(id: i32, pool: Arc<RwLock<VibingPool>>) -> Result<Track> {
        Ok(sqlx::query_as!(Track,
            r#"
            SELECT track_id AS id, path, title, author, genre, duration
            FROM tracks
            WHERE track_id = $1
            "#, id
        ).fetch_one(pool.read().await.get_inner()).await?)
    }

    pub async fn get_by_title(title: &str, pool: Arc<RwLock<VibingPool>>) -> Result<Track> {
        Ok(sqlx::query_as!(Track,
            r#"
            SELECT track_id AS id, path, title, author, genre, duration
            FROM tracks
            WHERE title = $1
            "#, title
        ).fetch_one(pool.read().await.get_inner()).await?)
    }

    pub async fn get_all(pool: Arc<RwLock<VibingPool>>) -> Result<Vec<Track>> {
        Ok(sqlx::query_as!(Track,
            r#"
            SELECT track_id AS id, path, title, author, genre, duration
            FROM tracks
            "#
        ).fetch_all(pool.read().await.get_inner()).await?)
    }

    pub async fn get_by_filter(filter: TrackFilter, pool: Arc<RwLock<VibingPool>>) -> Result<Vec<Track>> {
        let mut query_builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
            "
            SELECT DISTINCT t.track_id AS id, t.path, t.title, t.author, t.genre, t.duration
            FROM tracks t
            "
        );

        if let Some(ref vibes) = filter.vibes {
            if !vibes.is_empty() {
                query_builder.push(
                    "
                    JOIN tracks_with_vibes twv ON t.track_id = twv.track
                    JOIN vibes v ON twv.vibe = v.vibe_id
                    "
                );
            }
        }

        let mut where_added = false;
        let mut add_where = |builder: &mut QueryBuilder<sqlx::Postgres>| {
            if !where_added {
                builder.push(" WHERE ");
                where_added = true;
            } else {
                builder.push(" AND ");
            }
        };

        if let Some(pattern) = filter.pattern {
            add_where(&mut query_builder);
            query_builder.push("t.title ILIKE ");
            query_builder.push_bind(format!("%{}%", pattern));
        }

        if let Some(author) = filter.author {
            add_where(&mut query_builder);
            query_builder.push("t.author = ");
            query_builder.push_bind(author);
        }

        if let Some(vibes) = filter.vibes {
            if !vibes.is_empty() {
                add_where(&mut query_builder);
                query_builder.push("v.name = ANY(");
                query_builder.push_bind(vibes);
                query_builder.push(")");
            }
        }

        if let Some(order_by) = filter.order_by {
            // Whitelist columns to prevent SQL injection in ORDER BY
            let valid_columns = ["average_rating", "download_count"];
            if valid_columns.contains(&order_by.as_str()) {
                query_builder.push(format!(" ORDER BY t.{} DESC", order_by));
            }
        }

        if let Some(limit) = filter.limit {
            query_builder.push(" LIMIT ");
            query_builder.push_bind(limit as i64);
        }

        let query = query_builder.build_query_as::<Track>();
        Ok(query.fetch_all(pool.read().await.get_inner()).await?)
    }

    pub async fn apply_patch(mut self, patch: TrackPatch, pool: Arc<RwLock<VibingPool>>) -> Result<Track> {
        let change_path = (patch.path.is_some()) && (self.path != patch.path.clone().unwrap());
        let change_title = (patch.title.is_some()) && (self.title != patch.title);
        let change_author = (patch.author.is_some()) && (self.author != patch.author);
        let change_genre = (patch.genre.is_some()) && (self.genre != patch.genre);
        let change_duration = (patch.duration.is_some()) && (self.duration != patch.duration);

        if change_path {
            sqlx::query!(
                "
                UPDATE tracks
                SET path = $1
                WHERE track_id = $2
                ", patch.path.clone().unwrap(), self.id
            ).execute(pool.read().await.get_inner()).await?;

            self.path = patch.path.unwrap();
        }

        if change_title {
            sqlx::query!(
                "
                UPDATE tracks
                SET title = $1
                WHERE track_id = $2
                ", patch.title, self.id
            ).execute(pool.read().await.get_inner()).await?;

            self.title = patch.title;
        }

        if change_author {
            sqlx::query!(
                "
                UPDATE tracks
                SET author = $1
                WHERE track_id = $2
                ", patch.author, self.id
            ).execute(pool.read().await.get_inner()).await?;

            self.author = patch.author;
        }

        if change_genre {
            sqlx::query!(
                "
                UPDATE tracks
                SET genre = $1
                WHERE track_id = $2
                ", patch.genre, self.id
            ).execute(pool.read().await.get_inner()).await?;

            self.genre = patch.genre;
        }

        if change_duration {
            sqlx::query!(
                "
                UPDATE tracks
                SET duration = $1
                WHERE track_id = $2
                ", patch.duration, self.id
            ).execute(pool.read().await.get_inner()).await?;

            self.duration = patch.duration;
        }
        
        Ok(self)
    }

    pub async fn remove(self, pool: Arc<RwLock<VibingPool>>) -> Result<()> {
        sqlx::query!(
            "
            DELETE FROM tracks
            WHERE track_id = $1
            ", self.id
        ).execute(pool.read().await.get_inner()).await?;

        Ok(())
    }

    pub async fn count(pool: Arc<RwLock<VibingPool>>) -> Result<i64> {
        Ok(sqlx::query!(
            "
            SELECT COUNT(*) AS tracks_count
            FROM tracks
            "
        ).fetch_one(pool.read().await.get_inner()).await?
        .tracks_count
        .unwrap_or(-1))
    }

}

impl TrackFull {
    pub async fn create_from(metadata: TrackMetadata, pool: Arc<RwLock<VibingPool>>) -> Result<TrackFull> {
        let track = Track::create_from(metadata, pool).await?;

        Ok(TrackFull { track, vibes: Vec::new(), vote_count: 0, total_rating: 0, download_count: 0 })
    }

    pub async fn get_by_id(id: i32, pool: Arc<RwLock<VibingPool>>) -> Result<TrackFull> {
        let track = Track::get_by_id(id, pool.clone()).await?;

        let vibes = sqlx::query_as!(Vibe,
            "
            SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
            FROM tracks_with_vibes AS twv
            JOIN vibes AS vb ON twv.vibe = vb.vibe_id
            JOIN vibe_groups AS vg ON vb.vibe_group = vg.vibe_group_id
            WHERE twv.track = $1
            ", id
        ).fetch_all(pool.read().await.get_inner()).await?;

        let track_stat = sqlx::query!(
            "
            SELECT vote_count, total_rating, download_count
            FROM tracks
            WHERE track_id = $1
            ", id
        ).fetch_one(pool.read().await.get_inner()).await?;

        let vote_count = track_stat.vote_count;
        let total_rating = track_stat.total_rating;
        let download_count = track_stat.download_count;

        Ok(TrackFull { track, vibes, vote_count, total_rating, download_count })
    }

    pub async fn get_by_title(title: &str, pool: Arc<RwLock<VibingPool>>) -> Result<TrackFull> {
        let track = Track::get_by_title(title, pool.clone()).await?;

        let vibes = sqlx::query_as!(Vibe,
            "
            SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
            FROM tracks_with_vibes AS twv
            JOIN tracks AS tr ON twv.track = tr.track_id
            JOIN vibes AS vb ON twv.vibe = vb.vibe_id
            JOIN vibe_groups AS vg ON vb.vibe_group = vg.vibe_group_id
            WHERE tr.title = $1
            ", title
        ).fetch_all(pool.read().await.get_inner()).await?;

        let track_stat = sqlx::query!(
            "
            SELECT vote_count, total_rating, download_count
            FROM tracks
            WHERE title = $1
            ", title
        ).fetch_one(pool.read().await.get_inner()).await?;

        let vote_count = track_stat.vote_count;
        let total_rating = track_stat.total_rating;
        let download_count = track_stat.download_count;

        Ok(TrackFull { track, vibes, vote_count, total_rating, download_count })
    }

    pub async fn get_all(pool: Arc<RwLock<VibingPool>>) -> Result<Vec<TrackFull>> {
        let tracks = Track::get_all(pool.clone()).await?;

        let mut full_tracks = Vec::new();

        for track in tracks {
            let track_id = track.id;
            let vibes = sqlx::query_as!(Vibe,
                "
                SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
                FROM tracks_with_vibes AS twv
                JOIN tracks AS tr ON twv.track = tr.track_id
                JOIN vibes AS vb ON twv.vibe = vb.vibe_id
                JOIN vibe_groups AS vg ON vb.vibe_group = vg.vibe_group_id
                WHERE twv.track = $1
                ", track_id
            ).fetch_all(pool.read().await.get_inner()).await?;

            let track_stat = sqlx::query!(
                "
                SELECT vote_count, total_rating, download_count
                FROM tracks
                WHERE track_id = $1
                ", track_id
            ).fetch_one(pool.read().await.get_inner()).await?;

            let vote_count = track_stat.vote_count;
            let total_rating = track_stat.total_rating;
            let download_count = track_stat.download_count;

            full_tracks.push(
                TrackFull { track, vibes, vote_count, total_rating, download_count }
            );
        }

        Ok(full_tracks)
    }

    pub async fn get_by_filter(filter: TrackFilter, pool: Arc<RwLock<VibingPool>>) -> Result<Vec<TrackFull>> {
        let tracks = Track::get_by_filter(filter, pool.clone()).await?;
        let mut full_tracks = Vec::new();

        for track in tracks {
            let track_id = track.id;
            
            let vibes = sqlx::query_as!(Vibe,
                "
                SELECT vb.vibe_id AS id, vb.name, vg.name AS group_name
                FROM vibes vb
                JOIN vibe_groups vg ON vb.vibe_group = vg.vibe_group_id
                JOIN tracks_with_vibes twv ON vb.vibe_id = twv.vibe
                WHERE twv.track = $1
                ", track_id
            ).fetch_all(pool.read().await.get_inner()).await?;

            let track_stat = sqlx::query!(
                "
                SELECT vote_count, total_rating, download_count
                FROM tracks
                WHERE track_id = $1
                ", track_id
            ).fetch_one(pool.read().await.get_inner()).await?;

            full_tracks.push(TrackFull {
                track,
                vibes,
                vote_count: track_stat.vote_count,
                total_rating: track_stat.total_rating,
                download_count: track_stat.download_count,
            });
        }

        Ok(full_tracks)
    }

    pub async fn apply_patch(mut self, patch: FullTrackPatch, pool: Arc<RwLock<VibingPool>>) -> Result<TrackFull> {
        let track = if patch.track.is_some() {
            Track::apply_patch(self.track, patch.track.unwrap(), pool.clone()).await?
        } else {
            self.track
        };

        self.track = track;

        for remove_vibe in patch.remove_vibes {
            sqlx::query_as!(Vibe,
                "
                DELETE FROM tracks_with_vibes
                WHERE vibe = (
                    SELECT vibe_id
                    FROM vibes
                    WHERE name = $1
                )
                ", remove_vibe.1
            ).execute(pool.read().await.get_inner()).await?;
        
            self.vibes = self.vibes
                .into_iter()
                .filter(|vibe| vibe.name != remove_vibe.1)
                .collect();
        }

        for add_vibe in patch.add_vibes {
            let id = sqlx::query!(
                "
                INSERT INTO tracks_with_vibes (track, vibe)
                VALUES ($1, (
                    SELECT vibe_id
                    FROM vibes
                    WHERE name = $2
                ))
                RETURNING vibe
                ", self.track.id, add_vibe.1
            ).fetch_one(pool.read().await.get_inner()).await?
            .vibe;

            self.vibes.push(Vibe { id, name: add_vibe.1, group_name: add_vibe.0 });
        }

        if let Some(vote) = patch.new_vote {
            sqlx::query!(
                r#"
                UPDATE tracks
                SET
                    vote_count = vote_count + 1,
                    total_rating = total_rating + $1
                WHERE track_id = $2
                "#, vote as i64, self.track.id
            ).execute(pool.read().await.get_inner()).await?;

            self.vote_count += 1;
            self.total_rating += vote as i64;
        }

        if patch.new_download {
            sqlx::query!(
                r#"
                UPDATE tracks
                SET download_count = download_count + 1
                WHERE track_id = $1
                "#, self.track.id
            ).execute(pool.read().await.get_inner()).await?;

            self.download_count += 1;
        }

        Ok(self)
    }

    pub async fn remove(self, pool: Arc<RwLock<VibingPool>>) -> Result<()> {
        let id = self.track.id;
        Track::remove(self.track, pool.clone()).await?;

        sqlx::query!(
            "
            DELETE FROM tracks_with_vibes
            WHERE track = $1
            ", id
        ).execute(pool.read().await.get_inner()).await?;

        Ok(())
    }

    pub async fn count(pool: Arc<RwLock<VibingPool>>) -> Result<i64> {
        Track::count(pool).await
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SampleTrack {
    pub metadata: TrackMetadata,
    pub vibes: Vec<(String, String)>,
}

pub async fn sync_sample(sample_tracks: Vec<SampleTrack>, pool: Arc<RwLock<VibingPool>>) -> Vec<TrackFull> {
    let pool_guard = pool.read().await;
    let mut tx = pool_guard.transaction().await
        .expect("cannot get tx");

    let mut tracks: Vec<TrackFull> = Vec::new();

    for track in sample_tracks {
        let id = sqlx::query!(
            r#"
            INSERT INTO tracks (path, title, author, genre, duration)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING track_id
            "#, track.metadata.path, track.metadata.title, track.metadata.author, track.metadata.genre, track.metadata.duration
        ).fetch_one(&mut *tx).await
        .expect("cannot create track")
        .track_id;

        let new_track = Track {
            id,
            path: track.metadata.path,
            title: track.metadata.title,
            author: track.metadata.author,
            genre: track.metadata.genre,
            duration: track.metadata.duration,
        };
        
        let mut new_vibes: Vec<Vibe> = Vec::new();

        for vibe in track.vibes {
            sqlx::query!(
                "
                INSERT INTO tracks_with_vibes (track, vibe)
                VALUES ($1, (
                    SELECT vibe_id
                    FROM vibes
                    WHERE name = $2
                ))
                ", id, vibe.1
            ).execute(&mut *tx).await
            .expect("cannot add vibe");

            new_vibes.push(Vibe { id, name: vibe.1, group_name: vibe.0 });
        }

        tracks.push(TrackFull { track: new_track, vibes: new_vibes, vote_count: 0, total_rating: 0, download_count: 0 });
    }

    tx.commit().await.expect("cannot commit tx");

    tracks
}