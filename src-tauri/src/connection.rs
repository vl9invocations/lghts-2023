// use chrono::Duration;
use isahc::{
    // Error,
    ReadResponseExt,
};
// use ping::ping;
// use serde::{Deserialize, Serialize};
use serde_json::Value;
// use std::{
//     // fmt,
//     // net::{IpAddr, Ipv4Addr},
//     // ops::Deref,
//     // sync::Mutex,
//     // time::Duration,
// };
// const CONNECTION_URL: &str = "http://127.0.0.1:8008/zeroconf/";
const CONNECTION_URL: &str = "http://192.168.0.60:8081/zeroconf/";

// struct Light {
//     dev_id: String,
//     name: String,
//     switch: bool,
//     startup: bool,
//     pulse: bool,
//     pulse_width: u64,
//     ssid: String,
// }

pub async fn switch_on() -> bool {
    isahc::post(
        CONNECTION_URL.to_string() + "switch",
        "{
    \"deviceid\": \"\",
    \"data\": {
        \"switch\": \"on\"
    }
}",
    )
    .unwrap();
    return true;
}

pub async fn switch_off() -> bool {
    isahc::post(
        CONNECTION_URL.to_string() + "switch",
        "{
    \"deviceid\": \"\",
    \"data\": {
        \"switch\": \"off\"
    }
}",
    )
    .unwrap();

    return false;
}

// pub fn ping_device() -> () {
//     println!("{:?}", Some(pinger()));
// }

// pub fn pinger() -> Result<(), ping::Error> {
//     let device_ip_address = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 60));
//     let response = ping(
//         device_ip_address,
//         Some(Duration::from_millis(2000)),
//         Some(128),
//         Some(4),
//         Some(16),
//         Some(&[3; 24]),
//     );

//     response
// }

pub async fn get_switch_data() -> Result<Value, serde_json::Error> {
    // ping_device(); // questionable!
    isahc::post(
        CONNECTION_URL.to_string() + "info",
        "{ 
    \"deviceid\": \"\", 
    \"data\": {} 
}",
    )
    .unwrap()
    .json()
}
