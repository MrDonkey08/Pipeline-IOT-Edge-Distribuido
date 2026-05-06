use common::{SensorReading, current_timestamp_ms};
use rand::Rng;
use std::time::Duration;
use tokio::time;
use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .init();

    let edge_host =
        std::env::var("EDGE_HOST").unwrap_or_else(|_| "10.10.10.2".to_string());
    let edge_port =
        std::env::var("EDGE_PORT").unwrap_or_else(|_| "3001".to_string());
    let sensor_id =
        std::env::var("SENSOR_ID").unwrap_or_else(|_| "sensor1".to_string());
    let interval_ms: u64 = std::env::var("INTERVAL_MS")
        .unwrap_or_else(|_| "1000".to_string())
        .parse()
        .unwrap_or(1000);

    let edge_url = format!("http://{}:{}/data", edge_host, edge_port);
    let client = reqwest::Client::new();
    let mut rng = rand::thread_rng();
    let mut sequence = 0u64;

    info!("Sensor {} iniciado, enviando a {}", sensor_id, edge_url);

    let mut interval = time::interval(Duration::from_millis(interval_ms));

    loop {
        interval.tick().await;

        let value = 15.0 + rng.gen_range(0.0..25.0);

        let reading = SensorReading {
            sensor_id: sensor_id.clone(),
            timestamp_ms: current_timestamp_ms(),
            value,
            unit: "celsius".to_string(),
            sequence,
        };

        sequence += 1;

        match client.post(&edge_url).json(&reading).send().await {
            Ok(response) if response.status().is_success() => {
                info!(
                    " Enviado: temp={:.1}°C, seq={}",
                    value, reading.sequence
                );
            }
            Ok(response) => {
                error!(" Error HTTP: {}", response.status());
            }
            Err(e) => {
                error!(" Error de conexión: {}", e);
            }
        }
    }
}
