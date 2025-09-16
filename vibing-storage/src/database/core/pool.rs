use sqlx::{
    pool::PoolConnection,
    postgres::PgPoolOptions,
    Pool,
    Postgres,
    Transaction
};

use crate::{
    config,
    database::error::Result
};

pub struct VibingPool {
    connection_pool: Pool<Postgres>
}

// some Result type is uneccessary because failures here are fatal, which should terminate the service immediately

impl VibingPool {
    pub async fn get() -> Self {
        Self {
            connection_pool: PgPoolOptions::new()
                .max_connections(20)
                .connect(&config::database_url()).await
                .expect("Failed to connect to vibing-storage database")
        }
    }

    pub async fn init() -> Self {
        let pool = Self {
            connection_pool: PgPoolOptions::new()
                .max_connections(20)
                .connect(&config::database_url()).await
                .expect("Failed to connect to vibing-storage database")
        };

        pool.run_migrations().await;

        pool
    }

    async fn run_migrations(&self) {
        sqlx::migrate!("./migrations")
            .run(&self.connection_pool).await
            .expect("cannot migrate database");
    }

    pub fn get_inner(&self) -> &Pool<Postgres> {
        &self.connection_pool
    }

    /// Gets a connection from the connection pool
    pub async fn connection(&self) -> Result<PoolConnection<Postgres>> {
        let connection = self.connection_pool
            .acquire().await?;

        Ok(connection)
    }

    pub async fn transaction(&self) -> Result<Transaction<'_, Postgres>> {
        let transaction = self.connection_pool
            .begin().await?;

        Ok(transaction)
    }
}
