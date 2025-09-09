use serde::Deserialize;

use crate::database::{core::pool::VibingPool, entities::vibe::Vibe, error::Result};

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Track {
    pub id: i32,
    pub path: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub genre: Option<String>,
    pub duration: Option<i32>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct TrackPatch {
    pub path: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub genre: Option<String>,
    pub duration: Option<i32>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct TrackFull {
    pub track: Track,
    pub vibes: Vec<Vibe>,
}

pub type GroupName = String;
pub type VibeName = String;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct FullTrackPatch {
    pub track: Option<TrackPatch>,
    pub add_vibes: Vec<(GroupName, VibeName)>,
    pub remove_vibes: Vec<(GroupName, VibeName)>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct TrackMetadata {
    pub path: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub genre: Option<String>,
    pub duration: Option<i32>,
}

impl Track {
    pub async fn create_from(metadata: TrackMetadata, pool: &VibingPool) -> Result<Track> {
        Ok(sqlx::query_as!(Track,
            r#"
            INSERT INTO tracks (path, title, author, genre, duration)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING track_id AS id, path, title, author, genre, duration
            "#, metadata.path, metadata.title, metadata.author, metadata.genre, metadata.duration
        ).fetch_one(pool.get_inner()).await?)
    }

    pub async fn get_by_id(id: i32, pool: &VibingPool) -> Result<Track> {
        Ok(sqlx::query_as!(Track,
            r#"
            SELECT track_id AS id, path, title, author, genre, duration
            FROM tracks
            WHERE track_id = $1
            "#, id
        ).fetch_one(pool.get_inner()).await?)
    }

    pub async fn get_by_title(title: &str, pool: &VibingPool) -> Result<Track> {
        Ok(sqlx::query_as!(Track,
            r#"
            SELECT track_id AS id, path, title, author, genre, duration
            FROM tracks
            WHERE title = $1
            "#, title
        ).fetch_one(pool.get_inner()).await?)
    }

    pub async fn apply_patch(mut self, patch: TrackPatch, pool: &VibingPool) -> Result<Track> {
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
            ).execute(pool.get_inner()).await?;

            self.path = patch.path.unwrap();
        }

        if change_title {
            sqlx::query!(
                "
                UPDATE tracks
                SET title = $1
                WHERE track_id = $2
                ", patch.title, self.id
            ).execute(pool.get_inner()).await?;

            self.title = patch.title;
        }

        if change_author {
            sqlx::query!(
                "
                UPDATE tracks
                SET author = $1
                WHERE track_id = $2
                ", patch.author, self.id
            ).execute(pool.get_inner()).await?;

            self.author = patch.author;
        }

        if change_genre {
            sqlx::query!(
                "
                UPDATE tracks
                SET genre = $1
                WHERE track_id = $2
                ", patch.genre, self.id
            ).execute(pool.get_inner()).await?;

            self.genre = patch.genre;
        }

        if change_duration {
            sqlx::query!(
                "
                UPDATE tracks
                SET duration = $1
                WHERE track_id = $2
                ", patch.duration, self.id
            ).execute(pool.get_inner()).await?;

            self.duration = patch.duration;
        }
        
        Ok(self)
    }

    pub async fn remove(self, pool: &VibingPool) -> Result<()> {
        sqlx::query!(
            "
            DELETE FROM tracks
            WHERE track_id = $1
            ", self.id
        ).execute(pool.get_inner()).await?;

        Ok(())
    }

}

impl TrackFull {
    pub async fn create_from(metadata: TrackMetadata, pool: &VibingPool) -> Result<TrackFull> {
        let track = Track::create_from(metadata, pool).await?;

        Ok(TrackFull { track, vibes: Vec::new() })
    }

    pub async fn get_by_id(id: i32, pool: &VibingPool) -> Result<TrackFull> {
        let track = Track::get_by_id(id, pool).await?;

        let vibes = sqlx::query_as!(Vibe,
            "
            SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
            FROM tracks_with_vibes AS twv
            JOIN vibes AS vb ON twv.vibe = vb.vibe_id
            JOIN vibe_groups AS vg ON vb.vibe_group = vg.vibe_group_id
            WHERE twv.track = $1
            ", id
        ).fetch_all(pool.get_inner()).await?;

        Ok(TrackFull { track, vibes })
    }

    pub async fn get_by_title(title: &str, pool: &VibingPool) -> Result<TrackFull> {
        let track = Track::get_by_title(title, pool).await?;

        let vibes = sqlx::query_as!(Vibe,
            "
            SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
            FROM tracks_with_vibes AS twv
            JOIN tracks AS tr ON twv.track = tr.track_id
            JOIN vibes AS vb ON twv.vibe = vb.vibe_id
            JOIN vibe_groups AS vg ON vb.vibe_group = vg.vibe_group_id
            WHERE tr.title = $1
            ", title
        ).fetch_all(pool.get_inner()).await?;

        Ok(TrackFull { track, vibes })
    }

    pub async fn apply_patch(mut self, patch: FullTrackPatch, pool: &VibingPool) -> Result<TrackFull> {
        let track = if patch.track.is_some() {
            Track::apply_patch(self.track, patch.track.unwrap(), pool).await?
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
            ).execute(pool.get_inner()).await?;
        
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
            ).fetch_one(pool.get_inner()).await?
            .vibe;

            self.vibes.push(Vibe { id, name: add_vibe.1, group_name: add_vibe.0 });
        }

        Ok(self)
    }

    pub async fn remove(self, pool: &VibingPool) -> Result<()> {
        let id = self.track.id;
        Track::remove(self.track, pool).await?;

        sqlx::query!(
            "
            DELETE FROM tracks_with_vibes
            WHERE track = $1
            ", id
        ).execute(pool.get_inner()).await?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct SampleTrack {
    pub metadata: TrackMetadata,
    pub vibes: Vec<(String, String)>,
}

pub async fn sync_sample(sample_tracks: Vec<SampleTrack>, pool: &VibingPool) -> Vec<TrackFull> {
    let mut tx = pool.transaction().await
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

        tracks.push(TrackFull { track: new_track, vibes: new_vibes });
    }

    tx.commit().await.expect("cannot commit tx");

    tracks
}