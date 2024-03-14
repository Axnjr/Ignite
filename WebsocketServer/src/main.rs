use axum::{ routing::{ post, get } , Json };
use socketioxide::{ extract::{ Data, SocketRef } , SocketIo };

use serde::Deserialize;

use serde_json::{ json , Value };
use axum::extract::{ Path , Query , State };

use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);

    socket.on("JOIN", |socket: SocketRef, Data::<String>(room)| async move {
        info!("Received join: {:?}", room);
        let _ = socket.leave_all();
        let _ = socket.join(room.clone());
        socket.emit("OK", format!("Subscribed to group: {}", room))
    });
}

#[tokio::main]
async fn main() -> Result<(), _> {

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    let shared_state = io.clone();

    let app = axum::Router::new()
        .route("/ignite", post(req_handler))
        // .with_state(io)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                // .layer(layer)
            ,
        )
    ;

    info!("Starting server");

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?
    ;

    Ok(())

}

