use delay_timer::prelude::*;
use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Manager, Runtime, State,
};

use crate::constants::TIMER_TASK_ID;

pub struct CronJobBuilder(DelayTimer);

#[tauri::command]
fn set_task(state: State<CronJobBuilder>) {
    let body = || async {
        println!("create_async_fn_body!");

        println!("create_async_fn_body:i'success");
    };

    let mut task_builder = TaskBuilder::default();

    let task = task_builder
        .set_task_id(TIMER_TASK_ID)
        .set_frequency_repeated_by_cron_str("@secondly")
        .set_maximum_parallel_runnable_num(2)
        .spawn_async_routine(body)
        .unwrap();

    let _result = state.0.insert_task(task).unwrap();
}

#[tauri::command]
fn stop_task(state: State<CronJobBuilder>) {
    let _result = state.0.stop_delay_timer().unwrap();
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    return PluginBuilder::new("timer")
        .setup(|app| {
            let delay_builder = DelayTimerBuilder::default().build();
            app.manage(CronJobBuilder(delay_builder));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![set_task, stop_task])
        .build();
}
