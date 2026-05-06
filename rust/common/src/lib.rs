use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// Funcion utilitaria para timestamp en milisegundos
pub fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

// estructura que el edge recibe del sensor (HTTP POST /data)

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SensorReading {
    pub sensor_id: String,
    pub timestamp_ms: u64,
    pub value: f64,
    pub unit: String,
    pub sequence: u64,
}

// estructura que el edge envia al cordinador (HTTP POST /report)

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EdgeReport {
    pub edge_id: String,
    pub timestamp_ms: u64,
    pub window_avg: f64,
    pub anomaly_detected: bool,
    pub sample_count: u32,
    pub latency_ms: u64,
    pub readings_received: u64,
}

// estructura para heartbeat (opcional, para tolerancia a fallos)

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heartbeat {
    pub node_id: String,
    pub role: String,
    pub timestamp_ms: u64,
    pub uptime_secs: u64,
}

// nueva seccion de codigo CordStatus
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoordStatus {
    pub active_edges: Vec<String>,
    pub total_readings: u64,
    pub anomalies_last_min: u32,
    pub uptime_s: u64,
    pub throughput_msg_per_sec: f64,
}
