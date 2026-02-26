use anyhow::{Context, Error};
use sqlx::postgres::PgPoolOptions;
pub struct PostgresSchema {
    pool: Pool<Postgres>,
}
use sqlx::{Pool, Postgres};

impl PostgresSchema {
    pub async fn connect(&self, database_url: &str) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(50)
            .connect(database_url)
            .await
            .context("Could not connect to database_url")?;
        Ok(Self { pool })
    }
    // TODO CREATE METHODS TO EVALUATE SCHEMA
    // Create structs to format the functions on ::query_as
    pub async fn get_tables(&self) {
        let d = sqlx::query!(
            r#"
            SELECT
                *
                FROM information_schema.columns
                WHERE table_schema = 'public'
            ORDER BY table_name, ordinal_position;
            "#
        )
        .fetch_all(&self.pool)
        .await;
        if d.is_ok() {
            println!("{:#?}", d);
        }
    }
}
