use std::collections::HashMap;
use std::error::Error;
use std::num::NonZeroU32;
use std::sync::Arc;

use axum::debug_handler;
use axum::http::StatusCode;
use axum::routing::post;
use axum::Extension;
use axum::Json;
use axum::{response::IntoResponse, response::Redirect, routing::get, Router};
use tokio;
use tokio::sync::broadcast;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;
use tower_http::trace::{self, TraceLayer};

// tracing
use tracing::info;
use tracing::Level;

// serde
use serde;
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_string;

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
    title: String,
}

type SharedState = Arc<RwLock<AppState>>;

// TODO:
// - Title
//      - Įvedimo laukas, kuriame kažkas turi būti įvesta (kuris negali būti paliktas tuščias)
// - Add natural number cell type
//      - Įvedimo laukas, kuriame turi būti įvestas sveikas teigiamas skaičius
//      - HTML puslapio elementų paslėpimas/parodymas (ne išmetimas) panaudojant CSS savybę display, priklausomai nuo to, kas įvesta kokiame nors formos lauke (būtina naudoti jQuery biblioteką);
//      - Egzistuojančių žymių stiliaus pakeitimas (atributui style priskiriant naują reikšmę) (error out a cell on non-natural number)
// - Add an end-point for cell clear
//      - Egzistuojančių žymių išmetimas (pvz. ištrinti įvedimo lauke numeriu nurodytą paragrafą)
// - jquerysize cell insert
//      - Naujų žymių įterpimas (pvz. teksto gale pridėti paragrafą su tekstu, įvestu įvedimo lauke)
// - use polling for title updates
// - reintroduce websockets for cell updates
// - don't resend the same data

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
        .nest_service("/style.css", ServeDir::new("./front/style.css"))
        .nest_service("/script.js", ServeDir::new("./front/script.js"))
        .nest_service("/index.html", ServeDir::new("./front/index.html"))
        .route("/update", post(update_cell))
        .route("/update_title", post(update_title))
        .route("/poll_state", get(poll_state))
        .route("/poll_title", get(poll_title))
        .route("/", get(|| async { Redirect::permanent("index.html") }))
        .layer(trace_layer)
        .layer(Extension(Arc::new(RwLock::new(AppState {
            matrix: HashMap::new(),
            tx,
            title: "New sheet".to_owned(),
        }))));

    // let listener1 = tokio::net::TcpListener::bind("127.0.0.1:3000")
    //     .await
    //     .unwrap();
    // println!("listening on http://{:?}", listener1.local_addr()?);
    let listener2 = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("listening on http://{:?}", listener2.local_addr()?);

    axum::serve(listener2, app).await.unwrap();

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
    let _ = tx.send(CellUpdate {
        coordinate: req.coordinate,
        text: req.text,
    });

    StatusCode::OK
}

#[derive(Deserialize, Debug)]
struct UpdateTitleParams {
    title: String,
    id: NonZeroU32,
}

#[debug_handler]
async fn update_title(
    state: Extension<SharedState>,
    Json(req): Json<UpdateTitleParams>,
) -> impl IntoResponse {
    let mut state = state.write().await;

    info!("UPDATE: to title with: {:?}", req);
    state.title = req.title.trim().to_string() + "-" + req.id.to_string().as_ref();
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
    })
    .unwrap()
}

#[derive(Serialize, Debug)]
struct PollTitleResponse {
    title: String,
}

async fn poll_title(state: Extension<SharedState>) -> impl IntoResponse {
    let state = state.read().await;
    to_string(&PollTitleResponse { title: state.title.clone() }).unwrap()
}
