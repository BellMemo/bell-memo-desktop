use std::{fs::File, path::Path};

use rusqlite::Connection;

use crate::util::app_path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Self {
        let db_path = app_path::app_config_path().as_path().join("db.db");

        let path_str = db_path.as_path().to_str().unwrap();

        if !Path::new(path_str).exists() {
            File::create(path_str).unwrap();
        }

        let conn: Connection = Connection::open(db_path).unwrap();
        log::info!("db init success: {}", conn.is_autocommit());
        Database { conn: conn }
    }

    pub fn ping(&self) {
        let result = self.conn.path().unwrap();
        match Some(result) {
            Some(_) => {
                log::info!("db init success")
            }
            None => {
                log::error!("db init failed")
            }
        }
    }
}
