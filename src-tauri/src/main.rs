// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use std::sync::Mutex;

// use commands::get_light_statuses;
// use connection::get_switch_data;
// use serde_json::Value;
use state::LightState;
// use std::{borrow::Borrow, env, future::IntoFuture, sync::Mutex};
// use tokio::runtime::Handle;
// use tauri::State;

mod commands;
mod connection;
mod state;

fn main() {
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    runtime.block_on(async {
        tauri::Builder::default()
            .manage(LightState(false.into()))
            // .setup(|app| {
            //     println!("ss");
            //     Ok(())
            // })
            .invoke_handler(tauri::generate_handler![
                commands::turn_switch,
                commands::get_light_statuses,
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    })
}
