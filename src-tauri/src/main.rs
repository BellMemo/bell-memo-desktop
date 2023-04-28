#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use bell_memo_desktop::bootstrap::app;

fn main() {
    app::Application::run();
}
