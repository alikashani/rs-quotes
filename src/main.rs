mod handlers;

use std::env;

use axum::routing::{get, post, Router};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

const CONNS: u32 = 5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Server config
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // DB config
    let database_url = env::var("DATABASE_URL")
        .expect("missing DATABASE_URL env");
    let pool = PgPoolOptions::new()
        .max_connections(CONNS)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        .route("/", get(handlers::health))
        .route("/quotes", post(handlers::create_quote))
        .with_state(pool);

    let listener = TcpListener::bind(addr)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
