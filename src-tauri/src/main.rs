// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use state::LightState;
use std::env;
// use tauri::State;

mod commands;
mod connection;
mod state;

fn main() {
    tauri::Builder::default()
        .manage(LightState(false.into()))
        .invoke_handler(tauri::generate_handler![
            commands::get_switch_state,
            commands::turn_switch,
            commands::get_light_statuses
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
