use crate::database::{
    core::pool::VibingPool,
    entities::{Page, Paginate, vibe::Vibe},
    error::Result,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, QueryBuilder};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub type GroupName = String;
pub type VibeName = String;

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq, FromRow)]
pub struct Track {
    pub id: i32,
    pub path: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub genre: Option<String>,
    pub duration: Option<i32>,
    pub vote_count: i32,
    pub total_rating: i64,
    pub download_count: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq, FromRow)]
pub struct TrackFull {
    pub track: Track,
    pub vibes: Vec<Vibe>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct TrackFullPatch {
    pub path: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub genre: Option<String>,
    pub duration: Option<i32>,
    pub new_vote: Option<u8>,
    pub new_download: bool,
    pub add_vibes: Option<Vec<(GroupName, VibeName)>>,
    pub remove_vibes: Option<Vec<(GroupName, VibeName)>>,
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
    pub vibes: Option<Vec<VibeName>>,
    pub limit: Option<i32>,
    pub order_by: Option<String>,
}

impl TrackFull {
    pub async fn create_from(
        metadata: TrackMetadata,
        pool: Arc<RwLock<VibingPool>>,
    ) -> Result<TrackFull> {
        let track = sqlx::query_as!(
            Track,
            r#"
            INSERT INTO tracks (path, title, author, genre, duration)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING 
                track_id AS id, path, title, author, genre,
                duration, vote_count, total_rating, download_count
            "#,
            metadata.path,
            metadata.title,
            metadata.author,
            metadata.genre,
            metadata.duration
        )
        .fetch_one(pool.read().await.get_inner())
        .await?;

        Ok(TrackFull {
            track,
            vibes: Vec::new(),
        })
    }

    pub async fn get_by_id(id: i32, pool: Arc<RwLock<VibingPool>>) -> Result<TrackFull> {
        let pool_guard = pool.read().await;

        let track = sqlx::query_as!(
            Track,
            r#"
            SELECT
                track_id AS id, path, title, author, genre,
                duration, vote_count, total_rating, download_count
            FROM tracks
            WHERE track_id = $1
            "#,
            id
        )
        .fetch_one(pool_guard.get_inner())
        .await?;

        let vibes = sqlx::query_as!(
            Vibe,
            r#"
            SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
            FROM tracks_with_vibes AS twv
            JOIN vibes AS vb ON twv.vibe = vb.vibe_id
            JOIN vibe_groups AS vg ON vb.vibe_group = vg.vibe_group_id
            WHERE twv.track = $1
            "#,
            id
        )
        .fetch_all(pool_guard.get_inner())
        .await?;

        Ok(TrackFull { track, vibes })
    }

    pub async fn get_by_title(title: &str, pool: Arc<RwLock<VibingPool>>) -> Result<TrackFull> {
        let pool_guard = pool.read().await;

        let track = sqlx::query_as!(
            Track,
            r#"
            SELECT
                track_id AS id, path, title, author, genre,
                duration, vote_count, total_rating, download_count
            FROM tracks
            WHERE title = $1
            "#,
            title
        )
        .fetch_one(pool_guard.get_inner())
        .await?;

        let vibes = sqlx::query_as!(
            Vibe,
            r#"
            SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
            FROM tracks_with_vibes AS twv
            JOIN vibes AS vb ON twv.vibe = vb.vibe_id
            JOIN vibe_groups AS vg ON vb.vibe_group = vg.vibe_group_id
            WHERE twv.track = $1
            "#,
            track.id
        )
        .fetch_all(pool_guard.get_inner())
        .await?;

        Ok(TrackFull { track, vibes })
    }

    pub async fn get_all(pool: Arc<RwLock<VibingPool>>) -> Result<Vec<TrackFull>> {
        let pool_guard = pool.read().await;

        let tracks: Vec<Track> = sqlx::query_as!(
            Track,
            r#"
            SELECT
                track_id AS id, path, title, author, genre,
                duration, vote_count, total_rating, download_count
            FROM tracks
            "#
        )
        .fetch_all(pool_guard.get_inner())
        .await?;

        if tracks.is_empty() {
            return Ok(Vec::new());
        }

        let track_ids: Vec<i32> = tracks.iter().map(|track| track.id).collect();

        let vibes = sqlx::query_as!(
            Vibe,
            r#"
            SELECT vb.vibe_id AS id, vb.name AS name, vg.name AS group_name
            FROM tracks_with_vibes AS twv
            JOIN vibes AS vb ON twv.vibe = vb.vibe_id
            JOIN vibe_groups AS vg ON vb.vibe_group = vg.vibe_group_id
            WHERE twv.track = ANY($1)
            "#,
            &track_ids
        )
        .fetch_all(pool_guard.get_inner())
        .await?;

        let mut vibes_map: HashMap<i32, Vec<Vibe>> = HashMap::new();
        for vibe in vibes {
            vibes_map.entry(vibe.id).or_default().push(vibe);
        }

        let full_tracks = tracks
            .into_iter()
            .map(|track| {
                let vibes = vibes_map.remove(&track.id).unwrap_or_default();
                TrackFull { track, vibes }
            })
            .collect();

        Ok(full_tracks)
    }

    pub async fn get_by_filter(
        filter: TrackFilter,
        pool: Arc<RwLock<VibingPool>>,
    ) -> Result<Vec<TrackFull>> {
        let mut query_builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
            r#"
            SELECT DISTINCT
                t.track_id AS id, t.path, t.title, t.author, t.genre,
                t.duration, t.vote_count, t.total_rating, t.download_count
            FROM tracks t
            "#,
        );

        if let Some(ref vibes) = filter.vibes {
            if !vibes.is_empty() {
                query_builder.push(
                    r#"
                    JOIN tracks_with_vibes twv ON t.track_id = twv.track
                    JOIN vibes vb ON twv.vibe = vb.vibe_id
                    "#,
                );
            }
        }

        query_builder.push(" WHERE TRUE");

        if let Some(pattern) = filter.pattern {
            query_builder.push(" AND (t.title ILIKE ");
            query_builder.push_bind(format!("%{}%", pattern));
            query_builder.push(" OR t.author ILIKE ");
            query_builder.push_bind(format!("%{}%", pattern));
            query_builder.push(") ");
        }

        if let Some(author) = filter.author {
            query_builder.push(" AND t.author = ").push_bind(author);
        }

        if let Some(vibes) = &filter.vibes {
            if !vibes.is_empty() {
                query_builder
                    .push(" AND vb.name = ANY(")
                    .push_bind(vibes)
                    .push(")");
            }
        }

        if let Some(order_by) = filter.order_by {
            let valid_columns = ["rating", "most download"];
            if order_by == valid_columns[0] {
                query_builder.push(" ORDER BY (CASE WHEN t.vote_count > 0 THEN t.total_rating::FLOAT / t.vote_count ELSE 0 END) DESC");
            } else if order_by == valid_columns[1] {
                query_builder.push(" ORDER BY t.download_count DESC");
            } else {
                // invalid query, reject to prevent sql injection
            }
        }

        if let Some(limit) = filter.limit {
            query_builder.push(" LIMIT ").push_bind(limit as i64);
        }

        let pool_guard = pool.read().await;
        let tracks: Vec<Track> = query_builder
            .build_query_as()
            .fetch_all(pool_guard.get_inner())
            .await?;

        if tracks.is_empty() {
            return Ok(Vec::new());
        }

        let track_ids: Vec<i32> = tracks.iter().map(|t| t.id).collect();

        let vibe_rows = sqlx::query!(
            r#"
            SELECT twv.track, vb.vibe_id, vb.name, vg.name AS group_name
            FROM tracks_with_vibes AS twv
            JOIN vibes AS vb ON twv.vibe = vb.vibe_id
            JOIN vibe_groups AS vg ON vb.vibe_group = vg.vibe_group_id
            WHERE twv.track = ANY($1)
            "#,
            &track_ids
        )
        .fetch_all(pool_guard.get_inner())
        .await?;

        let mut vibes_map: HashMap<i32, Vec<Vibe>> = HashMap::new();
        for row in vibe_rows {
            let vibe = Vibe {
                id: row.vibe_id,
                name: row.name,
                group_name: row.group_name,
            };
            vibes_map.entry(row.track).or_default().push(vibe);
        }

        let full_tracks = tracks
            .into_iter()
            .map(|track| {
                let vibes = vibes_map.remove(&track.id).unwrap_or_default();
                TrackFull { track, vibes }
            })
            .collect();

        Ok(full_tracks)
    }

    pub async fn apply_patch(
        mut self,
        patch: TrackFullPatch,
        pool: Arc<RwLock<VibingPool>>,
    ) -> Result<TrackFull> {
        let pool_guard = pool.read().await;

        // --- 1. Handle track metadata updates (path, title, author, etc.) ---
        let mut update_query: QueryBuilder<sqlx::Postgres> =
            QueryBuilder::new("UPDATE tracks SET ");
        let mut separated = update_query.separated(", ");
        let mut has_updates = false;

        if let Some(path) = patch.path {
            separated
                .push("path = ")
                .push_bind_unseparated(path.clone());
            self.track.path = path;
            has_updates = true;
        }

        if let Some(title) = patch.title {
            separated
                .push("title = ")
                .push_bind_unseparated(title.clone());
            self.track.title = Some(title);
            has_updates = true;
        }

        if let Some(author) = patch.author {
            separated
                .push("author = ")
                .push_bind_unseparated(author.clone());
            self.track.author = Some(author);
            has_updates = true;
        }

        if let Some(genre) = patch.genre {
            separated
                .push("genre = ")
                .push_bind_unseparated(genre.clone());
            self.track.genre = Some(genre);
            has_updates = true;
        }

        if let Some(duration) = patch.duration {
            separated
                .push("duration = ")
                .push_bind_unseparated(duration);
            self.track.duration = Some(duration);
            has_updates = true;
        }

        if let Some(new_vote) = patch.new_vote {
            separated.push("vote_count = vote_count + 1");
            separated
                .push("total_rating = total_rating + ")
                .push_bind_unseparated(new_vote as i64);
            self.track.vote_count += 1;
            self.track.total_rating += new_vote as i64;
            has_updates = true;
        }

        if patch.new_download {
            separated.push("download_count = download_count + 1");
            self.track.download_count += 1;
            has_updates = true;
        }

        // Only execute update if there were changes to track metadata
        if has_updates {
            update_query
                .push(" WHERE track_id = ")
                .push_bind(self.track.id);
            update_query.build().execute(pool_guard.get_inner()).await?;
        }

        // --- 2. Handle vibe removal ---
        if let Some(remove_vibes) = patch.remove_vibes {
            if !remove_vibes.is_empty() {
                let mut query_builder =
                    QueryBuilder::new("DELETE FROM tracks_with_vibes WHERE track = ");
                query_builder.push_bind(self.track.id);
                query_builder.push(" AND vibe IN (SELECT vibe_id FROM vibes vb JOIN vibe_groups vg ON vb.vibe_group = vg.vibe_group_id WHERE (");

                let mut or_separated = query_builder.separated(" OR ");
                for (group, name) in &remove_vibes {
                    or_separated.push("(vg.name = ");
                    or_separated.push_bind_unseparated(group.clone());
                    or_separated.push_unseparated(" AND vb.name = ");
                    or_separated.push_bind_unseparated(name.clone());
                    or_separated.push_unseparated(")");
                }
                query_builder.push("))");

                query_builder
                    .build()
                    .execute(pool_guard.get_inner())
                    .await?;

                // Update local state
                let remove_set: std::collections::HashSet<(String, String)> =
                    remove_vibes.into_iter().collect();
                self.vibes
                    .retain(|v| !remove_set.contains(&(v.group_name.clone(), v.name.clone())));
            }
        }

        // --- 3. Handle vibe addition ---
        if let Some(add_vibes) = patch.add_vibes {
            if !add_vibes.is_empty() {
                // Fetch the full Vibe objects for the ones we need to add.
                let mut query = QueryBuilder::new(
                    r#"
                    SELECT vb.vibe_id as id, vb.name, vg.name as group_name
                    FROM vibes vb
                    JOIN vibe_groups vg ON vb.vibe_group = vg.vibe_group_id
                    WHERE 
                "#,
                );
                query.push("(");
                let mut or_separated = query.separated(" OR ");
                for (group, name) in &add_vibes {
                    or_separated.push("(vg.name = ");
                    or_separated.push_bind_unseparated(group.clone());
                    or_separated.push_unseparated(" AND vb.name = ");
                    or_separated.push_bind_unseparated(name.clone());
                    or_separated.push_unseparated(")");
                }
                query.push(")");

                let vibes_to_add = query
                    .build_query_as::<Vibe>()
                    .fetch_all(pool_guard.get_inner())
                    .await?;

                if !vibes_to_add.is_empty() {
                    // Build a single INSERT statement for the junction table
                    let mut insert_query =
                        QueryBuilder::new("INSERT INTO tracks_with_vibes (track, vibe) ");
                    insert_query.push_values(vibes_to_add.iter(), |mut b, vibe| {
                        b.push_bind(self.track.id);
                        b.push_bind(vibe.id);
                    });
                    insert_query.push(" ON CONFLICT (track, vibe) DO NOTHING");

                    insert_query.build().execute(pool_guard.get_inner()).await?;

                    // Update local state, avoiding duplicates
                    let existing_vibe_ids: std::collections::HashSet<i32> =
                        self.vibes.iter().map(|v| v.id).collect();
                    for vibe in vibes_to_add {
                        if !existing_vibe_ids.contains(&vibe.id) {
                            self.vibes.push(vibe);
                        }
                    }
                }
            }
        }

        Ok(self)
    }

    pub async fn remove(self, pool: Arc<RwLock<VibingPool>>) -> Result<()> {
        let pool_guard = pool.read().await;

        let mut tx = pool_guard.transaction().await?;

        let id = self.track.id;
        sqlx::query!(
            "
            DELETE FROM tracks
            WHERE track_id = $1
            ",
            id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            "
            DELETE FROM tracks_with_vibes
            WHERE track = $1
            ",
            id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn count(pool: Arc<RwLock<VibingPool>>) -> Result<i64> {
        Ok(sqlx::query!(
            "
            SELECT COUNT(*) AS tracks_count
            FROM tracks
            "
        )
        .fetch_one(pool.read().await.get_inner())
        .await?
        .tracks_count
        .unwrap_or(-1))
    }
}

pub struct TrackPaginationParams {
    pub page_num: i32,
    pub page_size: i32,
    pub filter: TrackFilter,
}

impl Paginate<TrackPaginationParams> for TrackFull {
    async fn page(
        params: &TrackPaginationParams,
        pool: Arc<RwLock<VibingPool>>,
    ) -> Result<Page<Self>> {
        // --- 1. Build the base query for both counting and fetching data ---
        let mut count_query_builder: QueryBuilder<sqlx::Postgres> =
            QueryBuilder::new("SELECT COUNT(DISTINCT t.track_id) as count FROM tracks t");
        let mut query_builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
            r#"
            SELECT DISTINCT
                t.track_id AS id, t.path, t.title, t.author, t.genre,
                t.duration, t.vote_count, t.total_rating, t.download_count
            FROM tracks t
            "#,
        );

        if let Some(vibes) = &params.filter.vibes {
            if !vibes.is_empty() {
                let join_sql = r#"
                    JOIN tracks_with_vibes twv ON t.track_id = twv.track
                    JOIN vibes vb ON twv.vibe = vb.vibe_id
                    "#;
                count_query_builder.push(join_sql);
                query_builder.push(join_sql);
            }
        }

        count_query_builder.push(" WHERE TRUE");
        query_builder.push(" WHERE TRUE");

        if let Some(pattern) = &params.filter.pattern {
            let pattern_sql = format!("%{}%", pattern);
            count_query_builder
                .push(" AND (t.title ILIKE ")
                .push_bind(pattern_sql.clone())
                .push(" OR t.author ILIKE ")
                .push_bind(pattern_sql.clone())
                .push(") ");
            query_builder
                .push(" AND (t.title ILIKE ")
                .push_bind(pattern_sql.clone())
                .push(" OR t.author ILIKE ")
                .push_bind(pattern_sql)
                .push(") ");
        }

        if let Some(author) = &params.filter.author {
            count_query_builder
                .push(" AND t.author = ")
                .push_bind(author.clone());
            query_builder
                .push(" AND t.author = ")
                .push_bind(author.clone());
        }

        if let Some(vibes) = &params.filter.vibes {
            if !vibes.is_empty() {
                count_query_builder
                    .push(" AND vb.name = ANY(")
                    .push_bind(vibes.clone())
                    .push(")");
                query_builder
                    .push(" AND vb.name = ANY(")
                    .push_bind(vibes.clone())
                    .push(")");
            }
        }

        // --- 2. Execute the COUNT query ---
        #[derive(Debug, FromRow)]
        struct Count {
            count: i64,
        }

        let pool_guard = pool.read().await;
        let total_items = count_query_builder
            .build_query_as::<Count>()
            .fetch_one(pool_guard.get_inner())
            .await?
            .count;

        if total_items == 0 {
            return Ok(Page::default());
        }

        // --- 3. Apply ordering and pagination to the main query ---
        if let Some(order_by) = &params.filter.order_by {
            let valid_columns = ["rating", "most download"];
            if order_by == valid_columns[0] {
                query_builder.push(" ORDER BY (CASE WHEN t.vote_count > 0 THEN t.total_rating::FLOAT / t.vote_count ELSE 0 END) DESC");
            } else if order_by == valid_columns[1] {
                query_builder.push(" ORDER BY t.download_count DESC");
            }
        }

        let limit = params
            .filter
            .limit
            .map_or(params.page_size, |l| l.min(params.page_size));
        query_builder.push(" LIMIT ").push_bind(limit);

        let offset = (params.page_num - 1) * params.page_size;
        query_builder.push(" OFFSET ").push_bind(offset);

        // --- 4. Execute the main query to get the items for the page ---
        let tracks: Vec<Track> = query_builder
            .build_query_as()
            .fetch_all(pool_guard.get_inner())
            .await?;

        if tracks.is_empty() {
            println!("empty case");
            return Ok(Page {
                total_items,
                total_page: (total_items as f64 / params.page_size as f64).ceil() as i32,
                page_num: params.page_num,
                page_size: params.page_size,
                ..Default::default()
            });
        }

        // --- 5. Fetch related vibes and construct the final TrackFull objects ---
        let track_ids: Vec<i32> = tracks.iter().map(|t| t.id).collect();
        let vibe_rows = sqlx::query!(
            r#"
            SELECT twv.track, vb.vibe_id, vb.name, vg.name AS group_name
            FROM tracks_with_vibes AS twv
            JOIN vibes AS vb ON twv.vibe = vb.vibe_id
            JOIN vibe_groups AS vg ON vb.vibe_group = vg.vibe_group_id
            WHERE twv.track = ANY($1)
            "#,
            &track_ids
        )
        .fetch_all(pool_guard.get_inner())
        .await?;

        let mut vibes_map: HashMap<i32, Vec<Vibe>> = HashMap::new();
        for row in vibe_rows {
            let vibe = Vibe {
                id: row.vibe_id,
                name: row.name,
                group_name: row.group_name,
            };
            vibes_map.entry(row.track).or_default().push(vibe);
        }

        let full_tracks: Vec<TrackFull> = tracks
            .into_iter()
            .map(|track| {
                let vibes = vibes_map.remove(&track.id).unwrap_or_default();
                TrackFull { track, vibes }
            })
            .collect();

        // --- 6. Construct the final Page object ---
        Ok(Page {
            items: full_tracks,
            total_items,
            total_page: (total_items as f64 / params.page_size as f64).ceil() as i32,
            page_num: params.page_num,
            page_size: params.page_size,
        })
    }
}
