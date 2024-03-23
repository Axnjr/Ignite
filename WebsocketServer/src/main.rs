mod polling;
mod structs;

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
use structs::MyAuthData;
use polling::broadcast_queue_messages;
use std::env;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;


fn join_handler(socket: SocketRef, Data(room): Data<String>, ack: AckSender) {
    println!("👀🤗🫡 Received Join Group Request for Room: {:?}", room);
    let _ = socket.leave_all();
    let _ = socket.join(room.clone());
    ack.send("Joined the group !!").ok(); 
}

async fn authenticate_clients(socket: SocketRef, auth: MyAuthData, db_sate: Pool<Postgres>) {

    let resp = sqlx::query(
        &format!(r#" SELECT * FROM userkeystatus WHERE apiKey = '{}'; "#, auth.token ))
        .fetch_one(&db_sate)
        .await
    ;

    if resp.is_err() {
        let _ = socket.emit("ERROR", "Invalid API Key");
        let _ = socket.disconnect();
        println!("User with Invalid API Key made a request ❌😐🤨");
        return;
    }

    broadcast_queue_messages(socket).await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();

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
        s.on("JOIN", join_handler);                        // Register a handler for the "JOIN" event
        authenticate_clients(s, auth, db).await; // Authenticate the client with their "auth.token"
    });

    let app = axum::Router::new()
        .with_state(io)
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .layer(layer)
    ;

    println!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}