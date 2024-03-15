use axum::{
    routing::{get, post},
    Error, Json,
};
use socketioxide::{
    extract::{Data, SocketRef, AckSender, State},
    SocketIo,
    
    
};

use dotenv::dotenv;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[derive(Debug, Deserialize)]
struct MyAuthData {
    token: String,
}

async fn on_connect(socket: SocketRef, Data(auth): Data<MyAuthData>, db_sate: State<Pool<Postgres>>  ) {

    match 
        sqlx::query(&format!(r#" SELECT * FROM userkeystatus WHERE apiKey = '{}'; "#, auth.token))
        .fetch_one(&*db_sate)
        .await 
    {
        Ok(_) => {

            let _ = socket.emit("OK", "Server Connected !");

            // socket.on("JOIN", |socket: SocketRef, Data::<String>(room)| {
            //     println!("Received join: {:?}", room);
            //     let _ = socket.leave_all();
            //     let _ = socket.join(room.clone());
            // });
        }

        Err(_) => {
            let _ = socket.disconnect();
        }
    };
        
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();

    let url = env::var("DB_URL").expect("DB Connection URL not found !!");
    let db_client = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("DB CONNECTION FAILED")
    ;

    let (layer, io) = SocketIo::builder().with_state(db_client).build_layer();

    io.ns("/", on_connect);

    // let shared_state = io.clone();

    let app = axum::Router::new()
        // .route("/ignite", post(req_handler))
        .with_state(io)
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .layer(layer)
    ;

    println!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
