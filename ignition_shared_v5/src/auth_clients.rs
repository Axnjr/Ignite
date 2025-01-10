use socketioxide::extract::SocketRef;

use sqlx::{Pool, Postgres, Row};

use crate::map::{add_user_to_hash, decrement_user_hits, get_user_from_hash, increment_user_connections};

use crate::structs::UserLimits; 

use crate::util::get_request_limit_from_plan_name;

pub async fn authenticate_clients(socket: SocketRef, auth: String, db_sate: Pool<Postgres>) {

    // - Check if user already exists in the hashmap 
    //      - If yes, 
    //          - check if the user has reached the request limit ?
    //              - if yes, disconnect the user
    //              - if no, decrement the request limit & increment the connection count
    // ----------------------------------------------------------- //
    // - If no, check if the user exists in the database
    //      - if yes, add the user to the hashmap
    //      - if no, disconnect the user
   
    if let Some(user) = get_user_from_hash(&auth) {
        if user.hits <= 0 {
            let _ = socket.emit("ERROR", "💥 Daily request Limit Reached 🥹");
            let _ = socket.disconnect();
            return;
        }
        increment_user_connections(&auth);
        decrement_user_hits(&auth);
    } 

    // else {
    //     add_user_to_hash(auth.clone(), UserLimits {
    //         hits: 100,
    //         connections: 10,
    //         plan: "Hobby".to_owned(),
    //     })
    // }

    else {
        match sqlx::query(&format!(r#" SELECT * FROM userkeystatus WHERE apiKey = '{}'; "#,auth))
            .fetch_one(&db_sate)
            .await
        {
            Ok(resp) => {
                let p = resp.try_get::<String, _>("plantype").unwrap();
                let u = UserLimits {
                    hits: get_request_limit_from_plan_name(&p),
                    connections: 1,
                    plan: p.to_string(),
                };
                add_user_to_hash(auth.clone(), u);
                println!("User with API Key: {} added to the hashmap ✌️", auth);
            }
            Err(_err) => {
                let _ = socket.emit("ERROR", "Invalid API Key");
                let _ = socket.disconnect();
                println!("User with Invalid API Key made a request 🤨");
                return;
            }
        }
    }

    
}