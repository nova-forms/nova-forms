use sqlx::{sqlite::SqliteConnectOptions, SqlitePool, Error};
use std::str::FromStr;
use crate::FileId;

pub struct FileStore {
    pool: SqlitePool,
}

impl FileStore {
    pub async fn new() -> Result<Self, Error> {
        let options = SqliteConnectOptions::from_str(&format!(
            "sqlite:{}",
            std::env::current_dir().unwrap().display()
        ))?
        .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;

        let mut transaction = pool.begin().await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS files (
                uuid TEXT PRIMARY KEY,
                file_name TEXT NOT NULL,
                content_type: TEXT,
                data BLOB NOT NULL,
            )
        "#,
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(Self {
            pool
        })
    }

    pub fn insert(
        &self,
        file_name: String,
        content_type: Option<String>,
        data: Vec<u8>,
    ) -> FileId {
        let id = FileId::new();

        /*sqlx::query(
            r#"
            INSERT INTO files (uuid, file_name, content_type, data)
            VALUES ($1, $2, $3, $4)
        "#,
        )
        .bind(&id.0)
        .bind(&file_name)
        .bind(&content_type)
        .bind(&data)
        .execute(&self.pool)
        .await?;*/

        id
    }
}

struct Settings {
    
}