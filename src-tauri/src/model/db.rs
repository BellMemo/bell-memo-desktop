use rusqlite::{Connection, Error, Params, Row, Statement, Transaction, TransactionBehavior};
use std::{fs::File, path::Path};

use super::sql;
use crate::constants::APP_NAME;
use crate::util::app_path;

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

impl Default for Database {
    fn default() -> Self {
        let db_path = app_path::app_config_path().join(APP_NAME).join("db.db");
        let path_str = db_path.as_path().to_str().unwrap();

        println!("{}", app_path::app_config_path().as_path().display());

        if !Path::new(path_str).exists() {
            File::create(path_str).unwrap();
        }

        let conn: Connection = Connection::open(db_path).unwrap();

        // 初始化表结构
        conn.execute(sql::CREATE_MEMO_DATA, []).unwrap();
        conn.execute(sql::CREATE_MEMO_TAG, []).unwrap();
        conn.execute(sql::CREATE_MEMO_TAG_DATA, []).unwrap();

        Database { conn: conn }
    }
}

impl Database {
    pub fn new() -> Self {
        let db_path = app_path::app_config_path().join(APP_NAME).join("db.db");
        let path_str = db_path.as_path().to_str().unwrap();

        println!("{}", app_path::app_config_path().as_path().display());

        if !Path::new(path_str).exists() {
            File::create(path_str).unwrap();
        }

        let conn: Connection = Connection::open(db_path).unwrap();

        // 初始化表结构
        conn.execute(sql::CREATE_MEMO_DATA, []).unwrap();
        conn.execute(sql::CREATE_MEMO_TAG, []).unwrap();
        conn.execute(sql::CREATE_MEMO_TAG_DATA, []).unwrap();

        Database { conn: conn }
    }

    pub fn ping(&self) -> bool {
        let result = self.conn.is_autocommit();
        log::info!("log ping is success {}", result);
        return result;
    }

    /**
     * alias connection.execute
     */
    #[warn(dead_code)]
    pub fn exec<P: Params>(&self, sql: &str, params: P) -> Result<usize, Error> {
        let result = self.conn.execute(sql, params);
        return result;
    }

    /**
     * alias connection.query_row
     */
    #[warn(dead_code)]
    pub fn query<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<T, Error>
    where
        P: Params,
        F: FnOnce(&Row<'_>) -> Result<T, Error>,
    {
        let result = self.conn.query_row(sql, params, f);
        return result;
    }

    /**
     * alias prepare
     */
    #[warn(dead_code)]
    pub fn prepare(&self, sql: &str) -> Result<Statement, Error> {
        return self.conn.prepare(sql);
    }

    /**
     * alias transaction
     */
    pub fn transaction(&self) -> Transaction {
        let tx = Transaction::new_unchecked(&self.conn, TransactionBehavior::Deferred).unwrap();
        return tx;
    }
}
