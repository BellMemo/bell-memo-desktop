use tauri::State;

use crate::plugin::db::Db;

#[tauri::command]
pub fn greet(state: State<Db>) {
    let conn = state.connection.lock().unwrap();
    let db = conn.get("db").unwrap();
    println!("aaa {}", db.ping());
}
