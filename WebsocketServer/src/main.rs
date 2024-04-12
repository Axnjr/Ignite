// mod polling;
mod structs;

use axum::extract::Path;
use axum::routing::get;
use tracing_subscriber::FmtSubscriber;
use socketioxide::{
    extract::{AckSender, Data, SocketRef, TryData}, SocketIo
};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};

use dotenv::dotenv;
use structs::{
    add_user_to_hash, decrement_user_hits, get_connection_limit_from_plan_name,
    get_request_limit_from_plan_name, get_user_from_hash, remove_user_from_hash, ClientMessage,
    JoinLeaveRequestData, MyAuthData, UserLimits,
};
use std::{env, net::SocketAddr};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::structs::{decrement_user_connections, get_user_hash_map, init_map, reset_all_users_values};

fn join_handler(socket: SocketRef, message: JoinLeaveRequestData, ack: AckSender) {
    println!("👀🤗🫡 Received Join Group Request for Room: {:?}", message);
    decrement_user_hits(&message.key);
    // let _ = socket.leave_all();
    let _ = socket.join(message.group_id.clone());
    ack.send("Joined the group !!").ok();
}

fn leave_handler(socket: SocketRef, Data(room): Data<JoinLeaveRequestData>, ack: AckSender) {
    println!("👀🤗🫡 Received Leave Group Request for Room: {:?}", room);
    decrement_user_hits(&room.key);
    let _ = socket.leave(room.group_id.clone());
    ack.send("Left the group !!").ok();
}

fn message_handler(socket: SocketRef, Data(message): Data<ClientMessage>, ack: AckSender) {
    if let Some(user) = get_user_from_hash(&message.key) {
        if user.hits <= 0 {
            let _ = socket.emit("ERROR", "💥 Daily Limit Reached 🥹");
            let _ = socket.disconnect();
            remove_user_from_hash(&message.key);
            return;
        }
        decrement_user_hits(&message.key);
        let _ = socket
            .within(message.group_id)
            .broadcast()
            .emit(message.event_name, &message.message);
    }
}

async fn authenticate_clients(socket: SocketRef, auth: String, db_sate: Pool<Postgres>) {

    // - Check if user already exists in the hashmap 
    //      - If yes, 
    //          - check if the user has reached the connection limit ?
    //              - if yes, disconnect the user
    //              - if no, decrement the connection limit
    // ----------------------------------------------------------- //
    // - If no, check if the user exists in the database
    //      - if yes, add the user to the hashmap
    //      - if no, disconnect the user
   
    if let Some(user) = get_user_from_hash(&auth) {
        if user.connections <= 0 {
            let _ = socket.emit("ERROR", "💥 Connection Limit Reached 🥹");
            let _ = socket.disconnect();
            return;
        }
        decrement_user_connections(&auth);
    } 

    else {
        match sqlx::query(&format!(r#" SELECT * FROM userkeystatus WHERE apiKey = '{}'; "#,auth))
            .fetch_one(&db_sate)
            .await
        {
            Ok(resp) => {
                let p = resp.try_get::<String, _>("plantype").unwrap();
                let u = UserLimits {
                    hits: get_request_limit_from_plan_name(&p),
                    connections: get_connection_limit_from_plan_name(&p) - 1,
                    plan: p.to_string(),
                };
                add_user_to_hash(auth.clone(), u);
            }
            Err(_err) => {
                let _ = socket.emit("ERROR", "Invalid API Key");
                let _ = socket.disconnect();
                println!("User with Invalid API Key made a request ❌😐🤨");
                return;
            }
        }
    }

    println!("CLIENT CONNECTED !!");

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    init_map();

    let url = env::var("DB_URL").expect("DB Connection URL not found ☠️❌😬😱");
    let db_client = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("DB CONNECTION FAILED ☠️❌😬😱");

    let db = db_client.clone();

    let (layer, io) = SocketIo::builder().with_state(db_client).build_layer();

    io.ns("/", |s: SocketRef, Data::<MyAuthData>(auth)| async move {

        // ----- Test event ----- //
        s.on("client to server event", |s: SocketRef| {
            let _ = s.emit("server to client event", "Received client to server event");
        });
        // ----- Test event ----- //

        s.on("MESSAGE", message_handler);

        // Register a handler for the "JOIN" event
        s.on("JOIN", |_s: SocketRef, Data::<String>(payload), ack: AckSender| async move {
            // let payload = serde_json::from_value::<JoinLeaveRequestData>(payload).unwrap();
            println!("👀🤗🫡 Received Join Group Request for Room: {:?}", payload);
            ack.send("Joined the group !!").ok();
            // join_handler(s, payload, ack);
        });     

        s.on("LEAVE", leave_handler);   // Register a handler for the "LEAVE" event
                                      
        // Authenticate the client with their "auth.token"
        authenticate_clients(s, auth.token, db).await;
    });

    let app = axum::Router::new()
        .with_state(io)
        .route(
            "/users",
            get(|| async move {
                println!("Test Route");
                get_user_hash_map().unwrap()
            })
        )
        .route(
            "/reset",
            get(|| async move {
                reset_all_users_values();
                "MAP RESETED !!"}
            )
        )
        .route("/upgrade", get(|Path(key): Path<String>| async move { key.to_string() }))
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .layer(layer);

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    let address = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server:v3 Started On Port: {}", address);

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // broadcast_queue_messages().await;

    Ok(())
}
