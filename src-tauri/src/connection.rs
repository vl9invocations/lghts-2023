use isahc::ReadResponseExt;

use serde_json::Value;

const CONNECTION_URL: &str = "http://127.0.0.1:8008/zeroconf/";
// const CONNECTION_URL: &str = "http://192.168.0.60:8081/zeroconf/";

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
