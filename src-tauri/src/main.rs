#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use bell_memo_desktop::{cmd, plugin, prepare};
use tauri::{Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

fn main() {
    let mut context = tauri::generate_context!();
    let tray_menu = SystemTrayMenu::new();

    let config = context.config_mut();

    prepare::prepare(config);

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let win = app.get_window("main").unwrap();
                let visible = win.is_visible().unwrap();
                if visible {
                    win.hide().unwrap();
                } else {
                    win.show().unwrap();
                    win.set_focus().unwrap();
                }
            }
            _ => {}
        })
        .plugin(plugin::config::init())
        .plugin(plugin::log::init())
        .plugin(plugin::timer::init())
        .plugin(plugin::db::init())
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cmd::memo_data::select_memo_data,
            cmd::memo_data::insert_memo_data,
            cmd::memo_data::delete_memo_data,
            cmd::memo_data::edit_memo_data,
            cmd::memo_tag::search_memo_tag,
            cmd::memo_tag::insert_memo_tag,
            cmd::sync::save_data,
            cmd::sync::import_data,
        ])
        .build(context)
        .expect("error while running tauri application")
        .run(|app, event| match event {
            tauri::RunEvent::WindowEvent {
                label,
                event: win_event,
                ..
            } => match win_event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    let win = app.get_window(label.as_str()).unwrap();
                    win.hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            },
            _ => {}
        });
}
