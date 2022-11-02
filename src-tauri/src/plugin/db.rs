use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Manager, Runtime,
};

use crate::model::db::Database;

#[derive(Default)]
pub struct Db {
    pub connection: Arc<Mutex<HashMap<String, Database>>>,
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    return PluginBuilder::new("sqlite")
        .setup(|app| {
            let db_instance = Database::new();
            db_instance.ping();

            app.manage(Db {
                connection: Default::default(),
            });

            let db_state = app.state::<Db>();

            db_state
                .connection
                .lock()
                .unwrap()
                .insert(String::from("db"), db_instance);

            Ok(())
        })
        .build();
}
