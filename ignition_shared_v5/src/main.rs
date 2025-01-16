// mod map;
mod auth_clients;
mod dashmap;
mod handlers;
mod logging;
mod macros;
mod structs;
mod tcp;
mod lru;
mod util;
mod io_register;

use dashmap::init_map;
use io_register::register_ws_io_handlers;
use tcp::initialize_tcp_router;
use logging::{log_and_panic, log_messages_to_log_file};

use tokio::signal;
use tracing_subscriber::FmtSubscriber;
use socketioxide::SocketIo;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::{env, net::SocketAddr};

struct Config {
    db_url: String,
    max_connections: u32,
    port: u16,
}

impl Config {
    fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok(); // Load .env variables
        Ok(Self {
            db_url: env::var("DB_URL")?,
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
            max_connections: env::var("MAX_DB_CONNECTIONS")?.parse().unwrap(),
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    init_map();

    let server_config = log_if_panic!(
        Config::from_env, 
        "Error reading enviroment variables !!"
    );

    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    let db_client = log_if_panic_async!(
        PgPoolOptions::new()
            .max_connections(server_config.max_connections)
            .connect(server_config.db_url.as_str()),
        "Failed to connect to the database. Check the connection URL."
    );

    // let (layer, io) =  SocketIo::builder().with_state(db_client).build_layer();
    let db                   = db_client.clone();
    let (layer, io) = SocketIo::new_layer();

    register_ws_io_handlers(&io, db);

    let app           = initialize_tcp_router(io, layer);
    let address   = SocketAddr::from(([0, 0, 0, 0], server_config.port));
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    log_message!("INFO", "Server:v5 Started On Port: {}", address);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
    ;

    Ok(())
}


async fn shutdown_signal() {

    // STOPED SERVER FROM TERMINAL USING Ctrl+C

    signal::ctrl_c()
        .await
        .expect("Failed to listen for shutdown signal");

    log_message!("DEBUG", "Server stopped due to Ctrl+c signal.");
}