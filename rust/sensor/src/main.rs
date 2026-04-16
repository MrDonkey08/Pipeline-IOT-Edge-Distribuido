use rand::Rng;
use serde::Serialize;
use std::{thread, time::Duration};

#[derive(Serialize)]
struct SensorReading {
    sensor_id: String,
    value: f32,
}

#[tokio::main]
async fn main() {
    loop {
        let value: f32 = rand::thread_rng().gen_range(20.0..30.0);

        let reading = SensorReading {
            sensor_id: "sensor-1".to_string(),
            value,
        };

        println!("Sensor enviando: {:?}", value);

        let _ = reqwest::Client::new()
            .post("http://edge:3001/data")
            .json(&reading)
            .send()
            .await;

        thread::sleep(Duration::from_secs(2));
    }
}
