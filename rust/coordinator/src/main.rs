use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json,
    Router,
};
use common::{CoordStatus, EdgeReport, Heartbeat, current_timestamp_ms};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tokio::time;
use tracing::{info, warn};

const HEARTBEAT_TIMEOUT_SECS: u64 = 10;

#[derive(Clone)]
struct EdgeInfo {
    last_heartbeat: u64,
    total_reports: u64,
    anomalies_count: u32,
    avg_latency_sum: u64,
    avg_latency_count: u64,
    _first_seen: SystemTime,
}

impl EdgeInfo {
    fn new(heartbeat_time: u64) -> Self {
        Self {
            last_heartbeat: heartbeat_time,
            total_reports: 0,
            anomalies_count: 0,
            avg_latency_sum: 0,
            avg_latency_count: 0,
            _first_seen: SystemTime::now(),
        }
    }
}

struct CoordinatorState {
    edges: HashMap<String, EdgeInfo>,
    total_readings: u64,
    anomalies_last_min: u32,
    start_time: SystemTime,
    last_metrics_reset: u64,
    readings_last_min: u64,
}

impl CoordinatorState {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
            total_readings: 0,
            anomalies_last_min: 0,
            start_time: SystemTime::now(),
            last_metrics_reset: current_timestamp_ms(),
            readings_last_min: 0,
        }
    }

    fn record_heartbeat(&mut self, edge_id: String, timestamp: u64) -> bool {
        let is_new = !self.edges.contains_key(&edge_id);
        let edge = self
            .edges
            .entry(edge_id.clone())
            .or_insert_with(|| EdgeInfo::new(timestamp));
        edge.last_heartbeat = timestamp;
        if is_new {
            info!(" Nuevo edge conectado: {}", edge_id);
        }
        is_new
    }

    fn record_report(&mut self, edge_id: String, report: EdgeReport) {
        self.total_readings += 1;
        self.readings_last_min += 1;

        let edge = self
            .edges
            .entry(edge_id)
            .or_insert_with(|| EdgeInfo::new(report.timestamp_ms));
        edge.total_reports += 1;
        if report.anomaly_detected {
            edge.anomalies_count += 1;
            self.anomalies_last_min += 1;
        }
        edge.avg_latency_sum += report.latency_ms;
        edge.avg_latency_count += 1;

        let now = current_timestamp_ms();
        if now - self.last_metrics_reset > 60000 {
            self.anomalies_last_min = 0;
            self.readings_last_min = 0;
            self.last_metrics_reset = now;
        }
    }

    fn get_active_edges(&self, now_ms: u64) -> Vec<String> {
        self.edges
            .iter()
            .filter(|(_, info)| now_ms - info.last_heartbeat < HEARTBEAT_TIMEOUT_SECS * 1000)
            .map(|(id, _)| id.clone())
            .collect()
    }

    fn get_dead_edges(&self, now_ms: u64) -> Vec<String> {
        self.edges
            .iter()
            .filter(|(_, info)| now_ms - info.last_heartbeat >= HEARTBEAT_TIMEOUT_SECS * 1000)
            .map(|(id, _)| id.clone())
            .collect()
    }

    fn throughput_msg_per_sec(&self) -> f64 {
        self.readings_last_min as f64 / 60.0
    }

    fn get_status(&self) -> CoordStatus {
        let now = current_timestamp_ms();
        CoordStatus {
            active_edges: self.get_active_edges(now),
            total_readings: self.total_readings,
            anomalies_last_min: self.anomalies_last_min,
            uptime_s: self.start_time.elapsed().unwrap_or(Duration::ZERO).as_secs(),
            throughput_msg_per_sec: self.throughput_msg_per_sec(),
        }
    }
}

#[derive(Clone)]
struct AppState {
    state: Arc<Mutex<CoordinatorState>>,
}

async fn handle_report(
    State(app): State<AppState>,
    Json(report): Json<EdgeReport>,
) -> impl IntoResponse {
    let mut state = app.state.lock().unwrap();
    state.record_report(report.edge_id.clone(), report.clone());
    info!(
        " Reporte de {}: avg={:.1}C, anomaly={}",
        report.edge_id, report.window_avg, report.anomaly_detected
    );
    StatusCode::OK
}

async fn handle_heartbeat(
    State(app): State<AppState>,
    Json(hb): Json<Heartbeat>,
) -> impl IntoResponse {
    let mut state = app.state.lock().unwrap();
    state.record_heartbeat(hb.node_id.clone(), hb.timestamp_ms);
    StatusCode::OK
}

async fn handle_metrics(State(app): State<AppState>) -> Json<CoordStatus> {
    Json(app.state.lock().unwrap().get_status())
}

async fn health_check() -> &'static str {
    "OK"
}

async fn dead_edge_detection(app: AppState) {
    let mut interval = time::interval(Duration::from_secs(5));
    let mut last_dead = Vec::new();

    loop {
        interval.tick().await;
        let now = current_timestamp_ms();
        let dead = {
            let state = app.state.lock().unwrap();
            state.get_dead_edges(now)
        };

        for edge in &dead {
            if !last_dead.contains(edge) {
                warn!(" EDGE CAIDO: {}", edge);
            }
        }
        for edge in &last_dead {
            if !dead.contains(edge) {
                info!(" EDGE RECONECTADO: {}", edge);
            }
        }
        last_dead = dead;
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .init();

    let listen_port: u16 = std::env::var("LISTEN_PORT")
        .unwrap_or_else(|_| "8002".to_string())
        .parse()
        .unwrap_or(8002);
    let http_port: u16 = std::env::var("HTTP_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    let state = Arc::new(Mutex::new(CoordinatorState::new()));
    let app_state = AppState { state: state.clone() };

    info!("Coordinator iniciado en puerto {}", listen_port);

    let app_reports = Router::new()
        .route("/report", post(handle_report))
        .route("/heartbeat", post(handle_heartbeat))
        .with_state(app_state.clone());

    let app_metrics = Router::new()
        .route("/metrics", get(handle_metrics))
        .route("/health", get(health_check))
        .with_state(app_state.clone());

    tokio::spawn(dead_edge_detection(app_state));

    let reports_listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", listen_port)).await?;
    let metrics_listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", http_port)).await?;

    info!("Servidor de métricas iniciado en puerto {}", http_port);

    tokio::select! {
        result = axum::serve(reports_listener, app_reports) => result?,
        result = axum::serve(metrics_listener, app_metrics) => result?,
    }

    Ok(())
}
