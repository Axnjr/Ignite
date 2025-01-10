mod state;

use axum::{
    routing::{post, get},
    Json
};

use serde::Deserialize;
use serde_json::{json, Value};

use axum::extract::{Path, Query, State};

use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};

use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, serde::Deserialize)]
struct MessageIn {
    room: String,
    text: String,
}

#[derive(serde::Serialize)]
struct Messages {
    messages: Vec<state::Message>,
}

#[derive(Debug, serde::Deserialize)]
struct IgniteRequest {
    group_id:Option<String>,
    event_name:Option<String>,
    message:Option<String>
}


async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);

    socket.on(
        "join",
        |socket: SocketRef, Data::<String>(room), store: State<state::MessageStore>| async move {
            info!("Received join: {:?}", room);
            let _ = socket.leave_all();
            let _ = socket.join(room.clone());
            let messages = store.get(&room).await;
            let _ = socket.emit("messages", Messages { messages });
        },
    );

    socket.on(
        "message",
        |socket: SocketRef, Data::<MessageIn>(data), store: State<state::MessageStore>| async move {
            info!("Received message: {:?}", data);

            let response = state::Message {
                text: data.text,
                user: format!("anon-{}", socket.id),
                date: chrono::Utc::now(),
            };

            store.insert(&data.room, response.clone()).await;

            let _ = socket.within(data.room).emit("message", response);
        },
    )
}

// async fn handler(axum::extract::State(io): axum::extract::State<SocketIo>) {
//     info!("handler called");
//     let _ = io.emit("hello", "world");
// }


async fn req_handler(
    payload:Json<IgniteRequest>,
    State(io):State<SocketRef>
) -> String {

    println!("REQUEST PAYLOD: {:?}", payload);

    let _ = io.join(payload.group_id);
    let _ = io.to(payload.group_id).emit(payload.event_name, payload.message);

    info!("BroadCasted the message to clients !!");
    
    String::from("ok")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let messages = state::MessageStore::default();

    let (layer, io) = SocketIo::builder().with_state(messages).build_layer();

    io.ns("/", on_connect);

    let shared_state = io.clone();

    let app = axum::Router::new()
        .route("/ignite", post(req_handler))
        .with_state(io)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    info!("Starting server");

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
