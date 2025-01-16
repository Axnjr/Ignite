use socketioxide::extract::SocketRef;
use sqlx::{Pool, Postgres, Row};
use crate::dashmap::{add_user_to_hash, decrement_user_hits, get_user_from_hash};
use crate::structs::UserLimits; 
use crate::{get_connection_limit_from_plan_name, get_request_limit_from_plan_name};

pub async fn authenticate_test_clients(socket: SocketRef, auth: &str, connections: usize){

    if let Some(mut user) = get_user_from_hash(auth) {

        if connections > get_connection_limit_from_plan_name(&user.plan) {
            let _ = socket.emit("ERROR", "Con-current connection Limit Reached !");
            let _ = socket.disconnect();
            return;
        }

        if user.hits <= 0 {
            let _ = socket.emit("ERROR", "💥 Daily request Limit Reached 🥹");
            let _ = socket.disconnect();
            return;
        }

        // increment_user_connections(auth);
        user.connections = connections;
        decrement_user_hits(auth, Some(user));
    } 

    else {
        add_user_to_hash(auth, UserLimits {
            hits: 100,
            connections: 1,
            plan: "Hobby".to_owned(),
            // lru_pos: usize::MIN
        })
    }
}

pub async fn authenticate_clients(socket: SocketRef, auth: &str, connections: usize, db_sate: Pool<Postgres>) {

    // - Check if user already exists in the hashmap 
    //      - If yes, 
    //          - check if the user has reached the request limit ?
    //              - if yes, disconnect the user
    //              - if no, decrement the request limit & increment the connection count
    // ----------------------------------------------------------- //
    // - If no, check if the user exists in the database
    //      - if yes, add the user to the hashmap
    //      - if no, disconnect the user
   
    if let Some(mut user) = get_user_from_hash(auth) {

        if connections > get_connection_limit_from_plan_name(&user.plan) {
            let _ = socket.emit("ERROR", "Con-current connection Limit Reached !");
            let _ = socket.disconnect();
            return;
        }

        if user.hits <= 0 {
            let _ = socket.emit("ERROR", "💥 Daily request Limit Reached 🥹");
            let _ = socket.disconnect();
            return;
        }

        // update the concurrent connection count for the user with given key / client_id
        user.connections = connections;
        decrement_user_hits(auth, Some(user));
    }
     

    else {
        match sqlx::query(&format!(r#" SELECT * FROM userkeystatus WHERE apiKey = '{}'; "#, auth))
            .fetch_one(&db_sate)
            .await
        {
            Ok(resp) => {

                let user_plan = resp.try_get::<String, _>("plantype").unwrap();

                let u = UserLimits {
                    hits: get_request_limit_from_plan_name(&user_plan),
                    connections: 1,
                    plan: user_plan.to_string(),
                    // lru_pos: usize::MIN,
                };

                add_user_to_hash(auth, u);
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