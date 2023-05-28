use axum::{Router, Server, routing::get};

#[tokio::main]

async fn main() {
    let router = Router::new()
    .route("/", get(root_get));

    let server = Server::bind(&"0.0.0.0:8081".parse().unwrap()).serve(router.into_make_service());

    let addr = server.local_addr();
    println!("Listening on {}", addr);

    server.await.unwrap();

    println!("Hello, world!");
}

async fn root_get() -> &'static str {
    "Hello from Axum"
}
