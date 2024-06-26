use aws_sdk_sqs::client::Client;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use aws_config::{ BehaviorVersion, SdkConfig };
use crate::Igniter;
use Igniter::IgniteReq;

async fn aws_sdk_config() -> SdkConfig {
    aws_config::defaults(BehaviorVersion::latest()).load().await
}

#[derive(Debug,Serialize, Deserialize)]
pub struct IgnitionResponse {
    status: i32,
    message: String,
}

pub async fn forward_request_to_ws_server(status: String, req: IgniteReq) -> Json<Value> {

    let sqs_client = Client::new(&aws_sdk_config().await);

    let sqsc1 = Arc::new(sqs_client.clone());
    let sqsc2 = Arc::new(sqs_client.clone());

    let req1 = Arc::new(req.clone());
    let req2 = Arc::new(req.clone());

    println!("SENDING CLIENT MESSAGE !! & status is: {:?}", status);

    if status.to_lowercase() == "ok".to_owned() {

        tokio::spawn(async move { // add the request to AUTH-SQS ..

            print!("SENDIGN MESSAGE TO THE AUTH-Q, THREAD - 1");

            let _ = sqsc1
                .send_message() 
                .queue_url("https://sqs.ap-south-1.amazonaws.com/736854829789/AuthQStandard")
                .message_body(&req1.key)
                .send()
                .await
                .expect("Error Sending Message to AUTH-SQS !!")
            ;
        });

        tokio::spawn(async move { // add the request to WS-SQS ..

            print!("SENDIGN MESSAGE TO THE WS-Q, THREAD - 2");

            let _ = sqsc2
                .send_message() 
                .queue_url("https://sqs.ap-south-1.amazonaws.com/736854829789/WSQ")
                .message_body(serde_json::to_string(&req2).unwrap())
                .send()
                .await
                .expect("Error Sending Message to WS-SQS !!")
            ;
        });

        return Json(json!({
            "status": 200,
            "message": "Request forwarded to the WS-Server".to_owned(),
        }));
    }

    // return access denied or ....
    return Json(json!({
        "status": 403,
        "message": "Request denied, daily limit reached".to_owned(),
    })); 
   
}