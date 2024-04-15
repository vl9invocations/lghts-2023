// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

mod commands;
mod connection;

// "http://192.168.0.60:8081/zeroconf/switch", << real url

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_switch_state,
            commands::turn_switch,
            commands::get_light_statuses
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
