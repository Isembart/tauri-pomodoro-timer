use std::{sync::Mutex, time::Duration};

use tauri::{AppHandle, Manager, State};

struct TimerWrapper(Mutex<timer::Timer>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let app_handle = app.handle();
            app.manage(TimerWrapper(Mutex::new(timer::Timer::new(
                app_handle.clone(),
            ))));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            resume_timer,
            pause_timer,
            setup_timer,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod timer;

#[tauri::command]
fn resume_timer(timer_state: State<TimerWrapper>, app: AppHandle) {
    println!("Called resume timer");
    let mut timer = timer_state.0.lock().unwrap();
    timer.resume(app);
}

#[tauri::command]
fn pause_timer(timer_state: State<TimerWrapper>, app: AppHandle) {
    println!("Called pause timer");
    let mut timer = timer_state.0.lock().unwrap();
    timer.pause(app);
}

#[tauri::command]
fn setup_timer(timer_state: State<TimerWrapper>, total_secs: u64, app: AppHandle) {
    println!("Called setup timer");
    let mut timer = timer_state.0.lock().unwrap();
    timer.setup(Duration::from_secs(total_secs), app);
}
