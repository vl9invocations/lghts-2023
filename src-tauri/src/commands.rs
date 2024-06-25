// use isahc::ReadResponseExt;
use crate::{connection, state::LightState};
use serde_json::Value;
use tauri::{Manager, State};

#[tauri::command]
pub async fn turn_switch(state: State<'_, LightState>) -> Result<(), ()> {
    let switch_state: String =
        connection::get_switch_data().await.unwrap()["data"]["switch"].to_string();
    if switch_state == String::from("\"on\"") {
        connection::switch_off().await;
        *state.0.lock().unwrap() = false;
        // println!("Switch off state: {:?}", *state);
        Ok(())
    } else {
        connection::switch_on().await;
        *state.0.lock().unwrap() = true;
        // println!("Switch on state: {:?}", *state);
        Ok(())
    }
}

#[tauri::command]
pub async fn get_light_statuses(
    app: tauri::Window,
    state: State<'_, LightState>,
) -> Result<String, ()> {
    // Send a GET request and wait for the response headers.
    // Must be `mut` so we can read the response body.
    let switch_response: Value = connection::get_switch_data().await.unwrap();
    let switch_state: &str = &switch_response["data"]["switch"].as_str().unwrap();
    // println!("{}", &switch_state);

    match switch_state {
        "on" => {
            *state.0.lock().unwrap() = true;
            app.emit_all("lightstatus", crate::state::Payload { message: 1 })
                .unwrap();
        }
        _ => {
            *state.0.lock().unwrap() = false;
            app.emit_all("lightstatus", crate::state::Payload { message: 0 })
                .unwrap();
        }
    }

    Ok(format!(
        "T:{}\n 
        SWITCH:{}",
        chrono::offset::Local::now().format("%F %T"),
        &switch_response["data"]["switch"].as_str().unwrap(),
    ))
}

// #[tauri::command]
// pub async fn get_initial_state() -> u8 {
//     let switch_response: Value = connection::get_switch_data().await.unwrap();
//     let switch_state: &str = &switch_response["data"]["switch"].to_string();

//     match switch_state {
//         "true" => 1,
//         _ => 0,
//     }
// }

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", "Default")
// }
