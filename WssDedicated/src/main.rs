use axum::{http::Method, routing::get};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use socketioxide::{
    extract::{ 
        AckSender, 
        Data, 
        SocketRef 
    },
    SocketIo,
};
use dotenv::dotenv;
use std::{env, net::SocketAddr};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use http::header::CONTENT_TYPE;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ClientPayload {
    event: String,
    room: String,
    message: String,
}

#[derive(Debug, Deserialize)]
pub struct MyAuthData {
    pub token: String,
}

fn join_handler(socket: SocketRef, Data(room): Data<String>, ack: AckSender) {
    println!("👀🤗🫡 Received Join Group Request for Room: {:?}", room);
    let _ = socket.leave_all();
    let _ = socket.join(room.clone());
    ack.send("Joined the group !!").ok(); 
}

async fn authenticate_clients(socket: SocketRef, auth: MyAuthData) {
    // check if auth.token is valid by comparing it an key present in ec2 volume or env
    // if valid, add socket to the group
    // else, send error message to the client
    let validation_token = env::var("VALIDATION_TOKEN").expect("ENVIROMENT VARIABLE NOT FOUND ☠️ ❌ 😱");

    if auth.token != validation_token {
        println!("Client Authentication Failed");
        let _ = socket.emit("ERROR", "Authentication Failed: `auth.token` doesnt match with what is present on the VM");
        let _ = socket.disconnect();
        return ;
    }

    let _ = socket.emit("CONNECTED", "Connection succesfull !");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();

    let (layer, io) = SocketIo::builder().build_layer();

    io.ns("/", |s: SocketRef| async move { // , Data::<MyAuthData>(auth)

        s.on("JOIN", join_handler);               // Register a handler for the "JOIN" event

        s.on("MESSAGE", |s: SocketRef, Data::<Value>(payload)| async move {

            let payload = serde_json::from_value::<ClientPayload>(payload).unwrap();

            // broadcast emit given message, as the given event, to all clients, in the given room
            println!("Received Message to be Broadcasted: {:#?}", payload);
            let _ = s.within(payload.room).emit(payload.event, payload.message);  
        });                     

        // authenticate_clients(s, auth).await;     // Authenticate the client with their "auth.token"
    });

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any)
    ;

    let app = axum::Router::new()
        .with_state(io)
        .route("/test", get(|| async move { println!("Test Route"); "Test Route" } ))
        // .layer(cors)//ServiceBuilder::new().layer(CorsLayer::permissive()))
        // .layer(layer)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );
        // .layer(cors)
    ;

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000)
    ;

    let address = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server Started On Port: {}", address);

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}