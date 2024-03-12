use serde::{Deserialize, Serialize};
use sqlx::{ Pool, Postgres, Row };
use axum::extract::Json;
use crate::Threads;
use Threads::forward_request_to_ws_server;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IgniteReq {
    pub group_id: String,
    pub event_name: String,
    pub message: String,
    pub key: String,
}

pub async fn igniter(req_body: Json<IgniteReq>, db_client: Pool<Postgres>) -> String {

    let ir = IgniteReq {
        group_id: req_body.group_id.to_owned(),
        event_name: req_body.event_name.to_owned(),
        message: req_body.message.to_owned(),
        key: req_body.key.to_owned(),
    };

    match sqlx::query(&format!(r#" SELECT * FROM userkeystatus WHERE apiKey = '{}'; "#, ir.key))
    .fetch_one(&db_client)
    .await {

        Ok(resp) => {
            let status = resp.try_get::<String, _>("status").unwrap();
            return forward_request_to_ws_server(status, ir).await;
        }

        Err(err) => {
            print!("Error OCCURED: {:?}", err);
            return format!("Internal Error: {:#?}", err);
        }
    }
}