use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_sqs::client::Client;

use socketioxide::{
    extract::{ AckSender, Bin, Data, SocketRef, State },
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

#[derive(Debug, Deserialize)]
struct Message {
    key: String,
    group_id: String,
    event_name: String,
    message: String
}

struct PollingCounters {
    null_counter: i32,
    full_counter: i32
}

impl PollingCounters {
    fn reset(&mut self) {
        self.null_counter = 0;
        self.full_counter = 0;
    }

    fn incre_null_counter(&mut self) {
        self.null_counter = self.null_counter + 1;
    }

    fn incre_full_counter(&mut self) {
        self.full_counter = self.full_counter + 1;
    }

    fn reset_null_counter(&mut self) {
        self.null_counter = 0;
    }

    fn reset_full_counter(&mut self) {  
        self.full_counter = 0;
    }
}

async fn broadcast_queue_messages(socket: SocketRef) {

    let sqs_client = Client::new(&aws_sdk_config().await);

    let mut counters = PollingCounters {
        null_counter: 0,
        full_counter: 5
    };

    let polling_factor : i32 = 3;
    let mut sleep_duration: u64 = 5;
    const SLEEP_DURATION_AT_START: u64 = 5;

    loop {

        let mes = sqs_client
            .receive_message()
            .queue_url("https://sqs.ap-south-1.amazonaws.com/736854829789/WSQ")
            .set_max_number_of_messages(Some(5))
            .send()
            .await
            .expect("Error recieving messages from Message Queue ☠️😱")
        ;

        // no messages are being recived increase the null_counter
        if mes.messages().len() == 0 {
            counters.incre_null_counter();
        }
        // if all messages i.e messages.len() == max_number_of_messages are recived then increase the full_counter
        else if mes.messages().len() == 5 {
            counters.incre_full_counter();
            sleep_duration = SLEEP_DURATION_AT_START;
        }
        // reset all counter if messages are recived in between 0 - 5
        else if mes.messages().len() > 0 && mes.messages().len() < 5 {
            counters.reset();
        }

         // if null_counter is greater than 5 then increase the sleep_duration by 5 for making less requests to SQS
         if counters.null_counter > polling_factor {
            sleep_duration = sleep_duration * polling_factor as u64;

            println!("CHANGED AT LINE 98 sleep_duration: {}", sleep_duration);

            counters.reset_null_counter();
            tokio::time::sleep(std::time::Duration::from_secs(sleep_duration)).await;
            continue;
        }

        if counters.full_counter > polling_factor {
            sleep_duration = sleep_duration / polling_factor as u64;

            println!("CHANGED AT LINE 109 sleep_duration: {}", sleep_duration);

            counters.reset_full_counter();
        }

        for message in mes.messages() {
            let mes = message.body().unwrap();
            let mes: Message = serde_json::from_str(mes).unwrap();

            println!("------------------------");
            println!("JSON MESSAGE: {:#?}", mes);
            println!("------------------------");

            let _ = socket
                .within(mes.key + "_" + &mes.group_id)
                .emit(mes.event_name, mes.message)
            ;

            let _ = sqs_client
                .delete_message()
                .queue_url("https://sqs.ap-south-1.amazonaws.com/736854829789/WSQ")
                .receipt_handle(message.receipt_handle().unwrap())
                .send()
                .await
                .expect("Error deleting message from Message Queue ☠️😱")
            ;

            println!("Message Deleted from Message Queue ✅👍🏻");
        }

        if sleep_duration > 3599 {
            sleep_duration = SLEEP_DURATION_AT_START;
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