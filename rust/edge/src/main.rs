//! ============================================================
//! EDGE NODE - Procesamiento intermedio IoT/Edge
//! Versión HTTP - Recibe del sensor por POST, reenvía al coordinator
//! ============================================================
//!
//! REQUISITOS CUMPLIDOS:
//! 1. Crear endpoint HTTP para recibir datos (POST /data)
//! 2. Recibir JSON del sensor
//! 3. Parsear JSON a estructura SensorReading
//! 4. Reenviar datos al coordinator (HTTP POST)
//! 5. Mostrar logs en consola con colores y detalles
//!
//! ============================================================

use axum::{
    extract::State, http::StatusCode, response::IntoResponse, routing::post,
    Json, Router,
};
use common::{current_timestamp_ms, EdgeReport, SensorReading};
use serde_json;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;

// ========== CONFIGURACIÓN ==========
const DEFAULT_COORDINATOR_HOST: &str = "10.10.10.1";
const DEFAULT_COORDINATOR_PORT: &str = "8001";
const DEFAULT_HTTP_PORT: u16 = 3001;
const DEFAULT_EDGE_ID: &str = "edge1";

// ========== ESTADO DEL EDGE ==========
struct EdgeState {
    total_readings: u64,
    last_sequence: u64,
    lost_messages: u64,
}

impl EdgeState {
    fn new() -> Self {
        Self {
            total_readings: 0,
            last_sequence: 0,
            lost_messages: 0,
        }
    }

    fn process_reading(&mut self, sequence: u64) -> u64 {
        if self.last_sequence > 0 && sequence > self.last_sequence + 1 {
            let lost = sequence - self.last_sequence - 1;
            self.lost_messages += lost;
            tracing::warn!(
                " Gap detectado: perdidos {} mensajes (seq {} -> {})",
                lost,
                self.last_sequence,
                sequence
            );
        }
        self.last_sequence = sequence;
        self.total_readings += 1;
        self.total_readings
    }

    fn total_readings(&self) -> u64 {
        self.total_readings
    }
}

// ========== ESTADO COMPARTIDO PARA AXUM ==========
#[derive(Clone)]
struct AppState {
    edge_id: String,
    coordinator_url: String,
    state: Arc<Mutex<EdgeState>>,
    http_client: reqwest::Client,
}

// ========== HANDLER DEL ENDPOINT /data ==========
async fn handle_sensor_data(
    State(state): State<AppState>,
    Json(reading): Json<SensorReading>,
) -> impl IntoResponse {
    let receive_time = current_timestamp_ms();
    let latency_ms = receive_time.saturating_sub(reading.timestamp_ms);

    let total_readings = {
        let mut edge_state = state.state.lock().unwrap();
        edge_state.process_reading(reading.sequence)
    };

    let report = EdgeReport {
        edge_id: state.edge_id.clone(),
        timestamp_ms: receive_time,
        window_avg: reading.value,
        anomaly_detected: reading.value > 35.0,
        sample_count: 1,
        latency_ms,
        readings_received: total_readings,
    };

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!(" RECIBIDO DEL SENSOR [HTTP POST /data]");
    println!("    sensor_id:    {}", reading.sensor_id);
    println!("    temperatura:  {:.1}°{}", reading.value, reading.unit);
    println!("    sequence:     {}", reading.sequence);
    println!("    latencia:     {}ms (desde generación)", latency_ms);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    match state
        .http_client
        .post(&state.coordinator_url)
        .json(&report)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            println!(" REENVIADO AL COORDINATOR [{}]", state.coordinator_url);
            println!("   edge_id:      {}", state.edge_id);
            println!("   valor_reenviado: {:.1}°C", reading.value);
            println!(
                "   anomaly:      {}",
                if reading.value > 35.0 { " Si" } else { "No" }
            );
            println!("   total_reenviados: {}", total_readings);
            println!("   status:       {}", response.status());
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
            (StatusCode::OK, "OK")
        }
        Ok(response) => {
            println!(
                " [ERROR] Coordinator respondió con error: {}",
                response.status()
            );
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
            (StatusCode::INTERNAL_SERVER_ERROR, "Coordinator error")
        }
        Err(e) => {
            println!(" [ERROR] No se pudo enviar al coordinator: {}", e);
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
            (StatusCode::INTERNAL_SERVER_ERROR, "Connection error")
        }
    }
}

// ========== HEALTH CHECK ==========
async fn health_check() -> &'static str {
    "OK"
}

// ========== FUNCIÓN PRINCIPAL ==========
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .init();

    let coordinator_host = std::env::var("COORDINATOR_HOST")
        .unwrap_or_else(|_| DEFAULT_COORDINATOR_HOST.to_string());
    let coordinator_port = std::env::var("COORDINATOR_PORT")
        .unwrap_or_else(|_| DEFAULT_COORDINATOR_PORT.to_string());
    let edge_id = std::env::var("EDGE_ID")
        .unwrap_or_else(|_| DEFAULT_EDGE_ID.to_string());
    let http_port: u16 = std::env::var("HTTP_PORT")
        .unwrap_or_else(|_| DEFAULT_HTTP_PORT.to_string())
        .parse()
        .unwrap_or(DEFAULT_HTTP_PORT);

    let coordinator_url =
        format!("http://{}:{}/report", coordinator_host, coordinator_port);

    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    let state = AppState {
        edge_id: edge_id.clone(),
        coordinator_url,
        state: Arc::new(Mutex::new(EdgeState::new())),
        http_client,
    };

    println!("══════════════════════════════════════════════════════════");
    println!("              EDGE NODE INICIADO (HTTP)                   ");
    println!("══════════════════════════════════════════════════════════");
    println!("  ID:      {:30}", edge_id);
    println!("  Escucha: HTTP 0.0.0.0:{:<26}", http_port);
    println!("  Endpoint: POST /data                                    ");
    println!("  Reenvía a: {:<37}", state.coordinator_url);
    println!("══════════════════════════════════════════════════════════");
    println!(
        "\n Esperando datos de sensores en http://localhost:{}/data\n",
        http_port
    );

    let app = Router::new()
        .route("/data", post(handle_sensor_data))
        .route("/health", axum::routing::get(health_check))
        .with_state(state);

    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", http_port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
