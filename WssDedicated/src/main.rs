// mod polling;
mod structs;

use axum::routing::get;
use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::{ 
        AckSender, 
        Data, 
        SocketRef 
    },
    SocketIo,
};

use dotenv::dotenv;
use structs::MyAuthData;
use std::{env, net::SocketAddr};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ClientPayload {
    event: String,
    room: String,
    message: String,
}

fn client_message_handler(socket: SocketRef, Data(payload): Data<ClientPayload>) {
    // broadcast emit given message, as the given event, to all clients, in the given room
    // println!("👀🤗🫡 Received Message: {:#?}", payload);
    // let _ = socket.within(payload.room).broadcast().emit(payload.event, payload.message);
    println!("👀🤗🫡 Received Message on event !!!! {:#?}", payload);
}

fn join_handler(socket: SocketRef, Data(room): Data<String>, ack: AckSender) {
    println!("👀🤗🫡 Received Join Group Request for Room: {:?}", room);
    let _ = socket.leave_all();
    let _ = socket.join(room.clone());
    ack.send("Joined the group !!").ok(); 
}

async fn authenticate_clients(socket: SocketRef, auth: MyAuthData) {

    // check if auth.token is valid by comparing it an key present in ec2 volume
    // if valid, add socket to the group
    // else, send error message to the client

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();

    let (layer, io) = SocketIo::builder().build_layer();

    io.ns("/", |s: SocketRef, Data::<MyAuthData>(auth)| async move {
        s.on("JOIN", join_handler);               // Register a handler for the "JOIN" event
        s.on("MESSAGE", client_message_handler);                     
        authenticate_clients(s, auth).await;     // Authenticate the client with their "auth.token"
    });

    let app = axum::Router::new()
        .with_state(io)
        .route("/test", get(|| async move { println!("Test Route"); "Test Route" } ))
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .layer(layer)
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