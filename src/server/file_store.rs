use crate::{FileId, FileInfo};
use sqlx::{sqlite::SqliteConnectOptions, Error, SqlitePool};
use std::str::FromStr;

/// A database storage for files.
#[derive(Clone)]
pub struct FileStore {
    pool: SqlitePool,
}

impl FileStore {
    /// Tries to create a new `FileStore`.
    pub async fn new() -> Result<Self, Error> {
        let mut path = std::env::current_dir().unwrap();
        path.push("data.db");

        println!("creating database under {}", path.display());

        let options = SqliteConnectOptions::from_str(&format!("sqlite://{}", path.display()))?
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS files (
                uuid TEXT PRIMARY KEY,
                file_name TEXT NOT NULL,
                content_type TEXT,
                data BLOB NOT NULL
            )
        "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    /// Insert a file into the database, returns a corresponding `FileId`.
    pub async fn insert(&self, file_info: FileInfo, data: Vec<u8>) -> Result<FileId, sqlx::Error> {
        let id = FileId::new();

        sqlx::query(
            r#"
            INSERT INTO files (uuid, file_name, content_type, data)
            VALUES ($1, $2, $3, $4)
        "#,
        )
        .bind(&id.to_string())
        .bind(file_info.file_name())
        .bind(file_info.content_type())
        .bind(&data)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    /// Get a file from the database by its `FileId`.
    pub async fn get(&self, id: FileId) -> Result<Option<(FileInfo, Vec<u8>)>, Error> {
        let record: Option<(String, Option<String>, Vec<u8>)> = sqlx::query_as(
            r#"
            SELECT file_name, content_type, data
            FROM files
            WHERE uuid = $1
        "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(|r| (FileInfo::new(r.0, r.1), r.2)))
    }
}
