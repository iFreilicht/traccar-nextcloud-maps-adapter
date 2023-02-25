use std::{collections::HashMap, fs::File};

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

use rocket::{
    http::Status,
    tokio::time::{sleep, Duration},
};
use serde::Deserialize;
use std::io::read_to_string;
use toml::from_str;

#[derive(FromForm, Debug)]
struct TraccarDeviceUpdate {
    id: String,
    timestamp: u64,
    lat: f32,
    lon: f32,
    speed: f32,
    bearing: f32,
    altitude: f32,
    accuracy: f32,
    batt: f32,
}

#[derive(Deserialize)]
struct Config {
    endpoint: String,
    user: String,
    password: String,
    tarpit_sleep_s: u64,
    devices: HashMap<String, String>,
}

lazy_static! {
    static ref CONFIG: Config =
        from_str(&read_to_string(File::open("config.toml").unwrap()).unwrap()).unwrap();
}

async fn tarpit_sleep<T>() -> Option<T> {
    sleep(Duration::from_secs(CONFIG.tarpit_sleep_s)).await;
    None
}

async fn get_device_name<'a>(id: String) -> Option<&'a str> {
    let device_name = match CONFIG.devices.get(&id) {
        Some(device_name) => device_name,
        None => {
            // Prevent brute-forcing the ID
            return tarpit_sleep().await;
        }
    };

    return Some(device_name);
}

#[post("/devices?<update..>")]
async fn update_device_location(update: TraccarDeviceUpdate) -> (Status, &'static str) {
    let device_name = match get_device_name(update.id).await {
        Some(device_name) => device_name,
        None => return (Status::Unauthorized, "Wrong"),
    };
    let client = reqwest::Client::new();
    let result = client
        .post(&CONFIG.endpoint)
        .basic_auth(&CONFIG.user, Some(&CONFIG.password))
        .query(&[("user_agent", device_name)])
        .query(&[("timestamp", update.timestamp)])
        .query(&[
            ("lat", update.lat),
            ("lng", update.lon),
            ("speed", update.speed),
            ("bearing", update.bearing),
            ("altitude", update.altitude),
            ("altitude", update.altitude),
            ("accuracy", update.accuracy),
            ("battery", update.batt),
        ])
        .send()
        .await;
    match result {
        Ok(succ) => {
            println!("{}", succ.url());
            (Status::Ok, "Success")
        }
        Err(err) => {
            dbg!("{}", err);
            (Status::BadGateway, "Failure")
        }
    }
}

#[launch]
fn rocket() -> _ {
    // This log line is only necessary to immediately parse the config file on startup
    println!("Will forward requests to {}", CONFIG.endpoint);
    rocket::build().mount("/", routes![update_device_location])
}
