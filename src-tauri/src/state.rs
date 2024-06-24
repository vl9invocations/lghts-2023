// use tauri::async_runtime::Mutex;
use std::sync::Mutex;

#[derive(Debug)]
pub struct LightState(pub Mutex<bool>);
