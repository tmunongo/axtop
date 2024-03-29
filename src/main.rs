use std::sync::{Arc, Mutex};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    http::Response,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router, Server,
};
use sysinfo::{CpuExt, System, SystemExt};

#[tokio::main]

async fn main() {
    let app_state = AppState::default();

    let router = Router::new()
        .route("/", get(root_get))
        .route("/index.njs", get(indexnjs_get))
        .route("/index.css", get(indexcss_get))
        .route("/api/cpus", get(cpus_get))
        .route("/realtime/cpus", get(realtime_cpus_get))
        .with_state(app_state.clone());

    // update CPU in the BG
    tokio::task::spawn_blocking(move || {
        let mut sys = System::new();
        loop {
            sys.refresh_cpu();
            let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
            {
                let mut cpus = app_state.cpus.lock().unwrap();
                *cpus = v;
            }

            std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
        }
    });

    let server = Server::bind(&"0.0.0.0:8081".parse().unwrap()).serve(router.into_make_service());

    let addr = server.local_addr();
    println!("Listening on {}", addr);

    server.await.unwrap();
}

#[derive(Default, Clone)]
struct AppState {
    cpus: Arc<Mutex<Vec<f32>>>,
}

async fn root_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();

    Html(markup)
}

async fn indexnjs_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.njs").await.unwrap();

    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(markup)
        .unwrap()
}

async fn indexcss_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.css").await.unwrap();

    Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(markup)
        .unwrap()
}

#[axum::debug_handler]
async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
    let v = state.cpus.lock().unwrap().clone();

    Json(v)
}

#[axum::debug_handler]
async fn realtime_cpus_get(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|ws: WebSocket| async { realtime_cpus_stream(state, ws).await })
}

async fn realtime_cpus_stream(app_state: AppState, mut ws: WebSocket) {
    loop {
        let payload = serde_json::to_string(&*app_state.cpus.lock().unwrap()).unwrap();

        ws.send(Message::Text(payload)).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

