use anyhow::{Context, Error};
use sqlx::{Error as SqlError, postgres::PgPoolOptions};
pub struct PostgresSchema {
    pool: Pool<Postgres>,
}
use sqlx::{Pool, Postgres};

#[derive(Debug, sqlx::FromRow)]
struct ColumnInfo {
    table_name: Option<String>,
}

impl PostgresSchema {
    pub async fn connect(database_url: &str) -> Result<Self, SqlError> {
        let pool = PgPoolOptions::new().connect(database_url).await?;
        Ok(Self { pool })
    }
    // TODO CREATE METHODS TO EVALUATE SCHEMA
    // Create structs to format the functions on ::query_as
    pub async fn get_tables(&self) {
        let pool = &self.pool;
        let cols = sqlx::query_as::<_, ColumnInfo>(
            r#"
                SELECT
                    table_name
                    FROM information_schema.columns
                    WHERE table_schema = 'public'
                ORDER BY table_name, ordinal_position;
                "#,
        )
        .fetch_all(pool)
        .await;
        println!("{:#?}", cols);
    }

    pub async fn migrate(&self, migration_path: &str) -> Result<(), sqlx::migrate::MigrateError> {
        println!("Reached");
        let res = sqlx::migrate::Migrator::new(std::path::Path::new(&migration_path))
            .await?
            .run(&self.pool)
            .await;
        match res {
            Ok(_) => {
                println!("schema is ok");
                Ok(())
            }
            Err(e) => {
                println!("schema is bad");
                Err(e)
            }
        }
    }
}
