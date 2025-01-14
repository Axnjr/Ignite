use crate::{
    dashmap::{get_user_hash_map, remove_user_from_hash, reset_all_users_values}, 
    structs::UpgradeKey,
    log_message, 
};
use axum::{extract::Query, routing::get, Router};
use socketioxide::{layer::SocketIoLayer, SocketIo};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use std::fs::read_to_string;

pub fn initialize_tcp_router(io: SocketIo, layer: SocketIoLayer) -> Router {
    axum::Router::new()
        .with_state(io)
        .route(
            "/users",
            get(|| async {
                log_message!("DEBUG", "`/users` Route invoked.");
                get_user_hash_map().unwrap()
            }),
        )
        .route(
            "/reset",
            get(|| async {
                log_message!("INFO", "User HashMap was RESET !!");
                reset_all_users_values();
                "MAP RESETED !!"
            }),
        )
        .route(
            "/upgrade",
            get(|Query(params): Query<UpgradeKey>| async move {
                log_message!("DEBUG", "User with key: `{}` upgraded !.", params.key);
                remove_user_from_hash(&params.key);
            }),
        )
        .route("/getlogs", get(|| async move {
            match read_to_string("server.log") {
                Ok(content) => {
                    log_message!("INFO", "Log file was read.");
                    content
                },
                Err(_) => {
                    log_message!("ERROR", "Unable to read Log file !");
                    "Unable to read Log file !".to_string()
                },
            }
        }))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        )
}
