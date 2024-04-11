mod polling;
mod structs;

use axum::routing::get;
use serde::ser;
use socketioxide::{
    extract::{ 
        AckSender, 
        Data, 
        SocketRef 
    },
    SocketIo,
};

use sqlx::{
    postgres::PgPoolOptions, 
    Pool, 
    Postgres
};

use dotenv::dotenv;
use structs::{insert_ws, ClientMessage, MyAuthData};
use polling::broadcast_queue_messages;
use std::{env,net::SocketAddr};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::structs::init_map;


fn join_handler(socket: SocketRef, Data(room): Data<String>, ack: AckSender) {
    println!("👀🤗🫡 Received Join Group Request for Room: {:?}", room);
    let _ = socket.leave_all();
    let _ = socket.join(room.clone());
    ack.send("Joined the group !!").ok(); 
}

fn leave_handler(socket: SocketRef, Data(room): Data<String>, ack: AckSender) {
    println!("👀🤗🫡 Received Leave Group Request for Room: {:?}", room);
    let _ = socket.leave(room.clone());
    ack.send("Left the group !!").ok();
}

// fn message_handler(socket: SocketRef, Data(message): Data<ClientMessage>, ack: AckSender) {
    
// }

async fn authenticate_clients(socket: SocketRef, auth: String, db_sate: Pool<Postgres>) {
    // let resp = sqlx::query(
    //     &format!(r#" SELECT * FROM userkeystatus WHERE apiKey = '{}'; "#, auth ))
    //     .fetch_one(&db_sate)
    //     .await
    // ;

    // if resp.is_err() {
    //     let _ = socket.emit("ERROR", "Invalid API Key");
    //     let _ = socket.disconnect();
    //     println!("User with Invalid API Key made a request ❌😐🤨");
    //     return;
    // }

    insert_ws(auth, socket);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();
    init_map();

    let url = env::var("DB_URL").expect("DB Connection URL not found ☠️❌😬😱");
    let db_client = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("DB CONNECTION FAILED ☠️❌😬😱")
    ;

    let db = db_client.clone();

    let (layer, io) = SocketIo::builder().with_state(db_client).build_layer();

    io.ns("/", |s: SocketRef, Data::<MyAuthData>(auth)| async move {

        // ----- Test event ----- //
        s.on("client to server event", |s: SocketRef| { 
            let _ = s.emit("server to client event","Received client to server event"); 
        });
        // ----- Test event ----- //

        // s.on("MESSAGE", message_handler);
        s.on("JOIN", join_handler);                        // Register a handler for the "JOIN" event
        s.on("LEAVE", leave_handler);                      // Register a handler for the "LEAVE" event
        // Authenticate the client with their "auth.token"
        authenticate_clients(s, auth.token, db).await; 
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
    println!("Server:v3 Started On Port: {}", address);

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    broadcast_queue_messages().await;

    Ok(())
}