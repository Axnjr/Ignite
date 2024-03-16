use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_sqs::client::Client;

use socketioxide::{
    extract::{ AckSender, Bin, Data, SocketRef, State },
    SocketIo,
};

use dotenv::dotenv;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::broadcast;
use std::env;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[derive(Debug, Deserialize)]
struct MyAuthData {
    token: String,
}

#[derive(Debug, Deserialize)]
struct Message {
    api_key: String,
    group: String,
    event_name: String,
    message: String
}

async fn broadcast_queue_messages(socket: SocketRef) {

    let sqs_client = Client::new(&aws_sdk_config().await);

    let mut stale_counter = 0;
    const SLEEP_DURATION_AT_START: u64 = 10;
    let mut sleep_duration: u64 = 10;

    loop {

        let mes = sqs_client
            .receive_message()
            .queue_url("https://sqs.ap-south-1.amazonaws.com/736854829789/WSQ")
            .set_max_number_of_messages(Some(5))
            .send()
            .await
            .expect("Error recieving messages from Message Queue ☠️😱")
        ;

        // no messages are being recived increase the stale_counter
        if mes.messages().len() == 0 {
            stale_counter = stale_counter + 1;
            continue;
        }

        // if stale_counter is greater than 10 then increase the sleep_duration by 10 for making less requests to SQS
        if stale_counter > 10 {
            sleep_duration = sleep_duration * 10;
            stale_counter = 0;
        }

        // however if we start to recieve all 5 messages then reset the sleep_duration to SLEEP_DURATION_AT_START
        if mes.messages().len() == 5 {
            sleep_duration = SLEEP_DURATION_AT_START;
        }

        for message in mes.messages() {
            let message_body = message.body().unwrap();
            let message_body_json: Message = serde_json::from_str(&message_body).unwrap();

            let _ = socket.within(message_body_json.api_key+&message_body_json.group).emit(message_body_json.event_name, message_body_json.message);

            let _ = sqs_client
                .delete_message()
                .queue_url("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX")
                .receipt_handle(message.receipt_handle().unwrap())
                .send()
                .await
                .expect("Error deleting message from Message Queue ☠️😱")
            ;

            println!("Message Deleted from Message Queue ✅👍🏻");
        }

        tokio::time::sleep(std::time::Duration::from_secs(sleep_duration)).await;
    }

}

async fn aws_sdk_config() -> SdkConfig {
    aws_config::defaults(BehaviorVersion::latest()).load().await
}

fn join_handler(socket: SocketRef, Data(room): Data<String>, ack: AckSender) {
    // socket.on("JOIN", |socket: SocketRef, Data::<String>(room), ack: AckSender, Bin(bin)| async move {
    println!("Received Join Group Request for Room: {:?}", room);
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

    let url = env::var("DB_URL").expect("DB Connection URL not found !!");
    let db_client = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("DB CONNECTION FAILED")
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






async fn on_connect( socket: SocketRef, Data(auth): Data<MyAuthData>, db_sate: State<Pool<Postgres>>, ) {
    /*
    * 
    * REGISTER ALL HANDLERS
    * AUTHENTICATE THE CONNECTED CLIENT WITH THEIR "auth.token"
    * IF UN-AUTHORIZED THEN DISCONNECT THE CLIENT
    * KEEP LISTENING TO SQS MESSAGES AND BROADCAST THEM TO THE RESPECTIVE CLIENT
    * 
    */
    let _ = socket.emit("OK", "Server Connected !");
}