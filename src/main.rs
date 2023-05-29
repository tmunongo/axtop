use std::sync::{Arc, Mutex};

use axum::{Router, Server, routing::get, extract::State, Json, response::{IntoResponse, Html}, http::Response};
use sysinfo::{System, SystemExt, CpuExt};

#[tokio::main]

async fn main() {
    let router = Router::new()
    .route("/", get(root_get))
    .route("/index.js", get(indexjs_get))
    .route("/api/cpus", get(cpus_get))
    .with_state(AppState {
        sys: Arc::new(Mutex::new(System::new())),
    });

let server = Server::bind(&"0.0.0.0:8081".parse().unwrap()).serve(router.into_make_service());

let addr = server.local_addr();
println!("Listening on {}", addr);

server.await.unwrap();
}

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

async fn root_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();

   Html(markup)
}

async fn indexjs_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.js").await.unwrap();

   Response::builder().header("content-type", "application/javascript;charset=utf-8")
   .body(markup)
   .unwrap()
}

async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
    let mut sys = state.sys.lock().unwrap();

    sys.refresh_cpu();

    let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
    
    // for (i, cpu) in sys.cpus().iter().enumerate() {
    //     let i = i + 1;
    //     let usage = cpu.cpu_usage();
        
    //     writeln!(&mut s,"CPU {i} {usage}%").unwrap();
    // }

    Json(v)
}