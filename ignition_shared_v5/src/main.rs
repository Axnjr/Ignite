mod map;
mod structs;
mod util;
mod handlers;
mod auth_clients;

use axum::{extract::Query, http::Method};

use axum::routing::get;

use tracing_subscriber::FmtSubscriber;

use socketioxide::{ 
    extract::{
        AckSender, 
        Data, 
        SocketRef
    }, 
    SocketIo 
};

use sqlx::postgres::PgPoolOptions;

use dotenv::dotenv;

use structs::{
    JoinLeaveRequestData, 
    MyAuthData,
    UpgradeKey
};

use map::{
    remove_user_from_hash, 
    get_user_hash_map,
    init_map,
    reset_all_users_values
};

use std::{
    env, 
    net::SocketAddr
};

use tower::ServiceBuilder;

use tower_http::cors::{Any, CorsLayer};

use crate::handlers::info_handler;

use crate::{
    auth_clients::authenticate_clients, 
    handlers::{
        join_handler, 
        leave_handler, 
        message_handler
    }
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();

    init_map();

    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    let url = env::var("DB_URL").expect("DB Connection URL not found ☠️❌😬😱");

    let db_client = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("DB CONNECTION FAILED ☠️❌😬😱");

    let db = db_client.clone();

    let (layer, io) = SocketIo::builder().with_state(db_client).build_layer();

    io.ns("/", |s: SocketRef, Data::<MyAuthData>(auth)| async move { //  Data::<MyAuthData>(auth)

        let _ = s.emit("LOG", "PING");
        // ----- Test event ----- //
        //      s.on("client to server event", |s: SocketRef| {
        //          let _ = s.emit("server to client event", "Received client to server event");
        //      });
        // ----- Test event ----- //

        // Register a handler for the "INFO" event
        s.on("INFO", |Data::<MyAuthData>(payload), ack: AckSender| async move {
            info_handler(payload, ack);
        });

        // Register a handler for the "MESSAGE" event
        s.on("MESSAGE", message_handler);

        // Register a handler for the "LEAVE" event
        s.on("LEAVE", leave_handler);   

        // Register a handler for the "JOIN" event
        s.on("JOIN", |s: SocketRef, Data::<JoinLeaveRequestData>(payload), ack: AckSender| async move {
            join_handler(s, payload, ack);
        });       

        // Authenticate the client with their "auth.token"
        authenticate_clients(s, auth.token, db).await;

    });

    let _cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any)
    ;

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
                    "MAP RESETED !!"
                }
            )
        )

        .route(
            "/upgrade", 
            get(|Query(params): Query<UpgradeKey>| async move { 
                remove_user_from_hash(&params.key);
        }))

        .layer(
            ServiceBuilder::new()
            .layer(CorsLayer::permissive())
            .layer(layer),
        );

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000)
    ;

    let address = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server:v5 Started On Port: {}", address);

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}