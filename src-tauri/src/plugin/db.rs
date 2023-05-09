use crate::model::db::Database;
use futures::lock::Mutex;
use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Manager, Runtime,
};

pub struct Db {
    pub connection: Mutex<Database>,
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    return PluginBuilder::new("sqlite")
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let db_instance = Database::new().await;
                app.manage(Db {
                    connection: Mutex::new(db_instance),
                });
                Ok(())
            })
        })
        .build();
}
