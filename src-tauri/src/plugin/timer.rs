use delay_timer::prelude::*;
use std::time::Duration;
use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Runtime,
};
use tokio::time::sleep;

async fn async_template() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.get("http://httpbin.org/get").send().await?;
    println!("{:#?}", res);
    Ok(())
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    return PluginBuilder::new("timer")
        .setup(|_app| {
            let _delay_timer = DelayTimer::new();
            let mut task_builder = TaskBuilder::default();

            let body = move || async move {
                async_template().await.expect("Request failed.");
                sleep(Duration::from_secs(3)).await;
                println!("create_async_fn_body:i'success");
            };

            let _task = task_builder
                .set_frequency_repeated_by_seconds(8)
                .set_task_id(2)
                .set_maximum_running_time(5)
                .spawn_async_routine(body)
                .unwrap();

            // delay_timer.add_task(task).ok();
            Ok(())
        })
        .build();
}
