use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Runtime,
};

// @TODO: should impl log func provider to js invoke
#[tauri::command]
async fn do_something<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<(), String> {
    println!("command called");
    Ok(())
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    return PluginBuilder::new("log")
        .setup(|app| {
            let log_dir = app.path_resolver().log_dir().unwrap();
            let stdout = ConsoleAppender::builder().build();

            let log_file = FileAppender::builder()
                .encoder(Box::new(PatternEncoder::new("{d} {l} {t} - {m}{n}\n")))
                .build(log_dir.as_path().join("app.log"))
                .unwrap();

            let mut config = Config::builder()
                .appender(Appender::builder().build("log_file", Box::new(log_file)))
                .build(
                    Root::builder()
                        .appender("log_file")
                        .build(LevelFilter::Error),
                )
                .unwrap();

            // 开发模式使用命令行输出
            if cfg!(debug_assertions) {
                config = Config::builder()
                    .appender(Appender::builder().build("stdout", Box::new(stdout)))
                    .build(Root::builder().appender("stdout").build(LevelFilter::Info))
                    .unwrap();
            }

            log4rs::init_config(config).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![do_something])
        .build();
}
