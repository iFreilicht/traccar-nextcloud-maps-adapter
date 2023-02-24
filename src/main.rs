#[macro_use]
extern crate rocket;

#[derive(FromForm, Debug)]
struct TraccarDeviceUpdate {
    id: u64,
    timestamp: u64,
    lat: f32,
    lon: f32,
    speed: f32,
    bearing: f32,
    altitude: f32,
    accuracy: f32,
    batt: f32,
}

const ENDPOINT: &str = std::env!("ENDPOINT");
const USER: &str = std::env!("USER");
const PASSWORD: &str = std::env!("PASSWORD");

#[post("/devices?<update..>")]
async fn update_device_location(update: TraccarDeviceUpdate) -> &'static str {
    let client = reqwest::Client::new();
    let result = client
        .post(ENDPOINT)
        .basic_auth(USER, Some(PASSWORD))
        .query(&[("user_agent", update.id), ("timestamp", update.timestamp)])
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
            "Success!"
        }
        Err(err) => {
            dbg!("{}", err);
            "Failure!"
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![update_device_location])
}
