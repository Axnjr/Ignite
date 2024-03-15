use socketioxide::{
    extract::{AckSender, Data, SocketRef, State},
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

async fn on_connect( socket: SocketRef, Data(auth): Data<MyAuthData>, db_sate: State<Pool<Postgres>>, ) {

    let resp = sqlx::query(
        &format!(r#" SELECT * FROM userkeystatus WHERE apiKey = '{}'; "#, auth.token )).fetch_one(&*db_sate).await
    ;

    // if resp.is_err() {
    //     let _ = socket.emit("ERROR", "Invalid API Key");
    //     let _ = socket.disconnect();
    //     println!("User with Invalid API Key made a request ❌😐🤨");
    //     return;
    // }
    
    let _ = socket.emit("OK", "Server Connected !");
    let _ = socket.emit("JOINED", "SS3DFRTGYHUJIDFGH JK");
   
    socket.on("JOIN", |socket: SocketRef, Data::<String>(room)| async move {
        println!("Received join: {:?}", room);
        let _ = socket.leave_all();
        let _ = socket.join(room.clone());
        let _ = socket.emit("JOINED", format!("Joined the group: {}", room));
    });

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let url = env::var("DB_URL").expect("DB Connection URL not found !!");
    let db_client = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("DB CONNECTION FAILED");

    let (layer, io) = SocketIo::builder().with_state(db_client).build_layer();

    io.ns("/", on_connect);

    // let shared_state = io.clone();

    let app = axum::Router::new()
        // .route("/ignite", post(req_handler))
        .with_state(io)
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .layer(layer);

    println!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
