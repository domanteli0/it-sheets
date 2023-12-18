use std::collections::HashMap;
use std::error::Error;
use std::os::unix::net::UnixListener;
use std::sync::Arc;

use axum::body::Body;
use axum::debug_handler;
use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use axum::extract::{ws, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::post;
use axum::Extension;
use axum::Json;
use axum::{response::IntoResponse, response::Redirect, routing::get, Router};
use serde;
use serde::Deserialize;
use serde_json::to_string;
use tokio;
use tokio::sync::broadcast;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;
use tower_http::trace::{self, TraceLayer};
use tracing::error;
use tracing::info;
use tracing::Level;
use uuid::Uuid;

//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};

#[derive(serde::Serialize, Clone, Debug)]
struct CellUpdate {
    coordinate: Coordinate,
    text: String,
}

#[derive(serde::Serialize, Clone, Debug)]
struct ManyCellUpdates {
    updates: Vec<CellUpdate>,
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash, Debug)]
struct Coordinate {
    col: u16,
    row: u16,
}

#[derive(Clone)]
struct AppState {
    matrix: HashMap<Coordinate, String>,
    tx: broadcast::Sender<CellUpdate>,
}

type SharedState = Arc<RwLock<AppState>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    let (tx, _) = broadcast::channel(10);

    let app = Router::new()
        .nest_service("/src", ServeDir::new("../front/src"))
        .nest_service("/style.css", ServeDir::new("../front/style.css"))
        .nest_service("/index.html", ServeDir::new("../front/index.html"))
        .route("/update", post(update_cell))
        .route("/poll_state", get(poll_state))
        .route("/", get(|| async { Redirect::permanent("index.html") }))
        .layer(trace_layer)
        .layer(Extension(Arc::new(RwLock::new(AppState {
            matrix: HashMap::new(),
            tx,
        }))));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{:?}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Deserialize)]
struct UpdateCellRequest {
    coordinate: Coordinate,
    text: String,
}

#[debug_handler]
async fn update_cell(
    state: Extension<SharedState>,
    Json(req): Json<UpdateCellRequest>,
) -> impl IntoResponse {
    let mut state = state.write().await;

    info!(
        "UPDATE:  to: {:?}  with: {:?}",
        req.coordinate,
        &req.text[..]
    );
    state.matrix.insert(req.coordinate, req.text.clone());

    let tx = state.tx.clone();
    tx.send(CellUpdate {
        coordinate: req.coordinate,
        text: req.text,
    });

    StatusCode::OK
}

async fn poll_state(state: Extension<SharedState>) -> impl IntoResponse {
    let state = state.read().await;
    to_string(&ManyCellUpdates {
        updates: state
            .matrix
            .clone()
            .into_iter()
            .map(|(coordinate, text)| CellUpdate { coordinate, text })
            .collect::<Vec<_>>(),
    }).unwrap()
}
