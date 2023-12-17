use std::error::Error;

use axum::{routing::get, Router, response::Redirect};
use std::net::SocketAddr;
use tokio;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .nest_service("/", ServeDir::new("../front/."));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {:?}", listener);

    axum::serve(listener, app).await?;

    Ok(())
}
