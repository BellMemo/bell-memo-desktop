use crate::model::db::Database;
use futures::lock::Mutex;
use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Manager, RunEvent, Runtime,
};

pub struct Db {
    pub connection: Mutex<Database>,
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    return PluginBuilder::new("sqlite")
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let db_instance = Database::new().await;

                db_instance.ping().await;

                app.manage(Db {
                    connection: Mutex::new(db_instance),
                });
                Ok(())
            })
        })
        .on_event(|app, event| {
            if let RunEvent::Exit = event {
                tauri::async_runtime::block_on(async move {
                    let db_state = app.state::<Db>();
                    let db = db_state.connection.lock().await;
                    db.close().await;
                });
            }
        })
        .build();
}
