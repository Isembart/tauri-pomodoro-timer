// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod timer;

fn main() {
    tauri_pomodoro_timer_lib::run()
}
