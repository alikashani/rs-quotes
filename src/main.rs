use axum::http;
use axum::routing::{get, Router};
use tokio::net::TcpListener;

async fn health() -> http::StatusCode {
    http::StatusCode::OK
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(health));

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(addr)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
