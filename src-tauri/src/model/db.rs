use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqlitePool;
use sqlx::{Connection, Sqlite};
use std::sync::Arc;
use std::{fs::File, path::Path};

use crate::constants::APP_NAME;
use crate::util::app_path;

use super::sql;

#[derive(Debug)]
pub struct Database {
    pool: Arc<SqlitePool>,
}

impl Database {
    pub async fn new() -> Self {
        let db_path = app_path::app_config_path().join(APP_NAME).join("db.db");
        let path_str = db_path.as_path().to_str().unwrap();

        println!("{}", app_path::app_config_path().as_path().display());

        if !Path::new(path_str).exists() {
            File::create(path_str).unwrap();
        }

        let pool = SqlitePool::connect(format!("sqlite:{}", db_path.to_str().unwrap()).as_str())
            .await
            .unwrap();

        Database { pool: Arc::new(pool) }
    }

    pub async fn init_db(&self) -> bool {
        let mut conn = self.pool.acquire().await.unwrap();

        let mut tx = conn.begin().await.unwrap();

        sqlx::query(sql::CREATE_MEMO_DATA)
            .execute(&mut tx)
            .await
            .unwrap();
        sqlx::query(sql::CREATE_MEMO_TAG)
            .execute(&mut tx)
            .await
            .unwrap();
        sqlx::query(sql::CREATE_MEMO_TAG_DATA)
            .execute(&mut tx)
            .await
            .unwrap();

        tx.commit().await.unwrap();
        return true;
    }

    pub async fn get_connection(&self) -> PoolConnection<Sqlite> {
        self.pool.clone().acquire().await.unwrap()
    }
}
