// use chrono::Duration;
use isahc::{Error, ReadResponseExt};
use ping::ping;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fmt,
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};

struct Light {
    dev_id: String,
    name: String,
    switch: bool,
    startup: bool,
    pulse: bool,
    pulse_width: u64,
    ssid: String,
}

#[derive(Serialize, Deserialize)]
struct Data {
    switch: String,
}

#[derive(Serialize, Deserialize)]
struct CallBody {
    device_id: String,
    data: Data,
}

pub enum LightState {
    ON,
    OFF,
}

impl fmt::Display for LightState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LightState::ON => write!(f, "on"),
            LightState::OFF => write!(f, "off"),
        }
    }
}

pub fn switch_light(state: LightState) -> bool {
    // let switch_action = isahc::post(
    //     "http://192.168.0.60:8081/zeroconf/switch",
    //     serde_json::to_value(CallBody {
    //         device_id: "".to_string(),
    //         data: Data {
    //             switch: LightState::ON.to_string(),
    //         },
    //     })
    //     .unwrap(),
    // );
    // println!("{:?}", &switch_action);

    if matches!(state, LightState::ON) {
        return true;
    } else {
        return false;
    }
}

pub fn switch_on() -> bool {
    isahc::post(
        // "http://127.0.0.1:8008/zeroconf/switch",
        "http://192.168.0.60:8081/zeroconf/switch",
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

pub fn switch_off() -> bool {
    isahc::post(
        "http://192.168.0.60:8081/zeroconf/switch",
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

// pub fn get_switch_data() -> Result<isah, serde_json::Error> {
pub fn get_switch_data() -> Result<Value, serde_json::Error> {
    // ping_device(); // questionable!
    isahc::post(
        // "http://127.0.0.1:8008/zeroconf/info",
        "http://192.168.0.60:8081/zeroconf/info",
        "{ 
                \"deviceid\": \"\", 
                \"data\": {} 
            }",
    )
    // .unwrap()
    // .json()
    // .unwrap()
}
