use std::{sync::Mutex, time::Duration};

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, State,
};

struct TimerWrapper(Mutex<timer::Timer>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_app,_args,_cwd|{}))
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let app_handle = app.handle();
            let mut timer = timer::Timer::new(app_handle.clone());
            timer.setup(Duration::from_secs(1500), app_handle.clone());
            app.manage(TimerWrapper(Mutex::new(timer)));

            //TRAY

            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {
                        println!("Menu {:?} not handled", event.id);
                    }
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            resume_timer,
            pause_timer,
            setup_timer,
            get_remaining,
        ])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while building tauri application")
    // .run(|app, event| match event {
    //     tauri::RunEvent::ExitRequested { api, .. } => {
    //         api.prevent_exit();
    //         for (_label, window) in app.webview_windows() {
    //             window.close().unwrap();
    //         }
    //     }
    //     _ => (),
    // });
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

#[tauri::command]
fn get_remaining(timer_state: State<TimerWrapper>) -> u64 {
    timer_state.0.lock().unwrap().get_remaining().as_secs()
}
