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
        Database { conn: conn }
    }

    pub fn ping(&self) -> bool {
        let result = self.conn.is_autocommit();
        println!("println ping is success {}", result);
        log::info!("log ping is success {}", result);
        return result;
    }
}
