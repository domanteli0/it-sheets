use std::error::Error;
use std::os::unix::net::UnixListener;

use axum::body::Body;
use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use axum::extract::{ws, State, WebSocketUpgrade};
use axum::response::Response;
use axum::Json;
use axum::{response::IntoResponse, response::Redirect, routing::get, Router};
use log::error;
use serde;
use serde_json::to_string;
use tokio;
use tower_http::services::ServeDir;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use uuid::Uuid;

//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};

#[derive(serde::Serialize)]
struct ConnectResponse {
    id: Uuid,
}

#[derive(serde::Serialize)]
struct CellUpdate {
    coordinates: Coordinates,
    text: String,
}

#[derive(serde::Serialize)]
struct Coordinates {
    col: u16,
    row: u16,
}

#[derive(Clone)]
struct AppState {
    users: Vec<Uuid>,
}

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

    let app = Router::new()
        .nest_service("/src", ServeDir::new("../front/src"))
        .nest_service("/style.css", ServeDir::new("../front/style.css"))
        .nest_service("/index.html", ServeDir::new("../front/index.html"))
        .route("/ws", get(handle_new_conn))
        .route("/", get(|| async { Redirect::permanent("index.html") }))
        .layer(trace_layer)
        .with_state(AppState { users: vec![] });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{:?}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn handle_new_conn(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_failed_upgrade(|error| error!("Failed to upgrade WebSocket connection:\n{:?}", error))
        .on_upgrade(|mut web_socket| async move {
            // web_socket
            //     .send(ws::Message::Text(
            //         to_string(&ConnectResponse { id: Uuid::new_v4() }).unwrap(),
            //     ))
            //     .await
            //     .unwrap();

            let (mut sender, mut receiver) = web_socket.split();

            sender
                .send(Message::Text(
                    to_string(&CellUpdate {
                        coordinates: Coordinates { col: 1, row: 1 },
                        text: String::from("Hello, world!"),
                    })
                    .unwrap(),
                ))
                .await.unwrap();

            // if let Some(msg) = web_socket.recv().await {
            //     match msg {
            //         Ok(Message::Text(event)) => {
            //             web_socket.send(Message::Text(to_string(
            //                 &CellUpdate {
            //                     coordinates: Coordinates {
            //                         col: 1,
            //                         row: 1,
            //                     },
            //                     text: String::from("Hello, world!")
            //                 }
            //             ).unwrap()
            //         )).await.unwrap()
            //         },
            //         Ok(_) => todo!(),
            //         Err(_) => todo!(),
            //     }
            // }
            ()
        })
}
