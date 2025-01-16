use colored::*;
use rust_socketio::{client::Client, ClientBuilder, Payload, RawClient};
use serde_json::{json, Value};
use soft_aes::aes::{aes_dec_ecb, aes_enc_ecb};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;

pub struct Ignition {
    url: String,
    api_key: String,
    encryption_key: Option<String>,
    socket: Arc<Mutex<Option<Client>>>,
    io: Arc<Mutex<Option<ClientBuilder>>>,
    group_id: Option<String>,
    padding: String,
}

pub fn error_log(message: &str) {
    println!("{}", message.red());
}

pub fn dev_log(message: &str) {
    println!("{}", message.cyan());
}

impl Ignition {

    fn new(connection_url: &str, key: &str, encryption_key: Option<String>) -> Self {
        
        let socket_builder = ClientBuilder::new(connection_url.to_owned() + "/" + key); 
        
        let socket = socket_builder.clone()
            .on("ERROR", |payload, _| {
                if let Payload::Text(err) = payload {
                    error_log(err[0].as_str().unwrap());
                }
            })
            .on("LOG", |payload, _| {
                if let Payload::Text(message) = payload {
                    dev_log(message[0].as_str().unwrap());
                }
            })
            .connect()
            .expect("ERROR CONNECTING WITH IGNITION SERVER !!")
        ;

        Ignition {
            url: connection_url.to_string(),
            api_key: key.to_string(),
            encryption_key,
            socket: Arc::new(Mutex::new(Some(socket))),
            io: Arc::new(Mutex::new(Some(socket_builder))),
            group_id: None,
            padding: String::from("PKCS7"),
        }
    }

    fn encrypt(&self, message: &[u8]) -> Vec<u8> {
        match &self.encryption_key {
            Some(key) => aes_enc_ecb(
                message, 
                key.as_bytes(), 
                Some(self.padding.as_str())
            )
                .expect("Encryption failed"),
            None => {
                error_log("Encryption key not found !");
                panic!("Encryption key not found !")
            },
        }
    }

    fn decrypt(&self, message: &[u8]) -> Vec<u8> {
        match &self.encryption_key {
            Some(key) => aes_dec_ecb(
                message, 
                key.as_bytes(), 
                Some(self.padding.as_str())
            )
                .expect("Decryption failed"),
            None => {
                error_log("Encryption key not found !");
                panic!("Encryption key not found !")
            },
        }
    }

    async fn subscribe(&self, group_id: &str) {
        dev_log("Attempting to subscribe to given Room !");
        let io = self.socket.lock().await;
        if let Some(socket) = io.as_ref() {
            let _ = socket.emit_with_ack(
                "JOIN", 
                json!({
                    "key": self.api_key,
                    "group_id": format!("{}_{}", self.api_key, group_id),
                }),
                Duration::from_secs(2),
                |message: Payload, _| {
                    if let Payload::Text(mes) = message {
                        dev_log(mes[0].as_str().unwrap());
                    }
                }
            );
        }
    }

    async fn unsubscribe(&self, group_id: &str) {
        dev_log("Attempting to subscribe to given Room !");
        let io = self.socket.lock().await;
        if let Some(socket) = io.as_ref() {
            let _ = socket.emit_with_ack(
                "LEAVE", 
                json!({
                    "key": self.api_key,
                    "group_id": format!("{}_{}", self.api_key, group_id),
                }),
                Duration::from_secs(2),
                |message: Payload, _| {
                    if let Payload::Text(mes) = message {
                        dev_log(mes[0].as_str().unwrap());
                    }
                }
            );
        }
    }

    async fn emit(&self, event_name: &str, group_id: &str, message: &str) {
        let io = self.socket.lock().await;
        if let Some(socket) = io.as_ref() {
            let _ = socket.emit(event_name, json!({
                "event": event_name,
                "room": group_id, // format!("{}_{}", self.api_key, self.group_id.as_ref().unwrap_or(&"".to_string())),
                "message": self.encrypt(message.as_bytes()),
            }));
        }
    }

    async fn on<F>(&self, event_name: &str, mut call_back: F) where F: FnMut(&str) {
        let io = self.io.lock().await.clone();
        tokio::spawn(async move {
            if let Some(temp_io) = io.as_ref() {
                temp_io.on(event_name, move |data: Payload, socket| {
                    call_back(self.decrypt(data[0]));
                });
            }
        });
    }

}
