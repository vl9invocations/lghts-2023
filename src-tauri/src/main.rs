// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use std::sync::Mutex;

use commands::{get_initial_state, get_light_statuses};
// use commands::get_light_statuses;
// use connection::get_switch_data;
// use serde_json::Value;
use state::LightState;
use tauri::async_runtime::block_on;
// use std::{borrow::Borrow, env, future::IntoFuture, sync::Mutex};
// use tokio::runtime::Handle;
// use tauri::State;

mod commands;
mod connection;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .manage(LightState(false.into()))
        // .setup(|app| {
        //     async {
        //         let st = get_initial_state().await;
        //         println!("{}", st);
        //     };
        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![
            commands::turn_switch,
            commands::get_light_statuses,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
