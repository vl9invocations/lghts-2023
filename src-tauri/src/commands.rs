// use isahc::ReadResponseExt;
use serde_json::Value;

use crate::connection::{self, LightState};

#[tauri::command]
pub async fn turn_switch() -> bool {
    match connection::get_switch_data()["data"]["switch"]
        .as_str()
        .unwrap()
    {
        "on" => connection::switch_on(),
        "off" => connection::switch_off(),
        _ => false,
    }
}

#[tauri::command]
pub fn get_light_statuses() -> String {
    // Send a GET request and wait for the response headers.
    // Must be `mut` so we can read the response body.
    let response: Value = connection::get_switch_data().unwrap();

    format!(
        "T:{}\n SWITCH:{}",
        chrono::offset::Local::now().format("%F %T"),
        // &response["data"],
        &response["data"]["switch"].as_str().unwrap(),
    )
}

#[tauri::command]
pub fn get_switch_state() -> String {
    connection::get_switch_data().unwrap()["data"]["switch"]
        .as_str()
        .unwrap()
        .to_string()
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", "Default")
// }
