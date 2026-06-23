use axum::{http::StatusCode, routing::get, Router};
use axum::response::IntoResponse;

async fn health_check() -> impl IntoResponse{
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/health_check", get(health_check));

    let addr = "0.0.0.0:8000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

