use axum::{ 
    routing::{get, post}, 
    Router, 
    extract::Json,
    http::{Method, HeaderValue }
};
use socketioxide::{
    extract::{Data, SocketRef, State},
    SocketIo,
};
use serde::Deserialize;
use serde_json::Value;
use tower_http::cors::{CorsLayer, AllowMethods};
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use tower::ServiceBuilder;



#[derive(Debug,Deserialize)]
struct IgniteRequest {
    group_id: Vec<String>,
    event_name: String,
    message: Value,
}

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);

    socket.on("join", |socket: SocketRef, Data::<String>(data)| {
        info!("JOIN: {:?}", data);
        let _ = socket.leave_all();
        let _ = socket.join(data);
        
    });

    socket.on("message", |socket: SocketRef, Data::<Value>(data)| async move {
        info!("MESSAGE RECIEVED: {:?}", data);

        let rooms = socket.rooms().unwrap();

        #[derive(serde::Serialize)]
        struct MessageOut {
            user: String,
            text: String,
        }

        let mes = MessageOut {
            user: "server".to_string(),
            text: rooms.join(", "),
        };

        let _ = socket.emit("message", mes);
    });
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (layer, io) = SocketIo::new_layer();
    io.ns("/", on_connect); //|socket: SocketRef| async move { info!("Connected with Client {} !", socket.id) });

    let shared_state = io.clone();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/ignite", post({
            move |body| handler(body, shared_state)
        }))
        // .with_state(io)
        .layer( 
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        )
    ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}


async fn handler(
    Json(payload): Json<IgniteRequest>,
    io: SocketIo
) -> String {
    
    info!("POST HANDLER CALLED, GOT JSON PAYLOAD: {:?}", payload);

    #[derive(serde::Serialize)]
    struct MessageOut {
        user: String,
        text: String,
    }
    let message = MessageOut {
        user: "python".to_string(),
        text: "Hello from rust".to_string(),
    };

    let _ = io.clone().join(payload.group_id.clone());
    let _ = io.to(payload.group_id).emit("message", message);

    String::from("Message braodcasted succesfully !!")
}