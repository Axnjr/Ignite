use sqlx::Postgres;
use sqlx::Pool;
use socketioxide::SocketIo;
use socketioxide::extract::{AckSender, Data, SocketRef};
use crate::structs::{JoinLeaveRequestData, MyAuthData};
use crate::auth_clients::authenticate_clients;
use crate::handlers::{info_handler, join_handler, leave_handler, message_handler};
use crate::log_message;
use crate::auth_clients::authenticate_test_clients;


pub fn register_ws_io_handlers(io: &SocketIo, db: Pool<Postgres>) {

    // # FOR TESTING BOTH CLOSURES CANT WORK AT SAME TIME BECAUSE I DONT WANT TO CLONE 'io' AGAIN !!
    let _ = io.dyn_ns("/testing/{client_id}", |s: SocketRef, io: SocketIo| async move {

        let auth_token = s.ns().to_string();

        // # RESOLVED
        // untill version 15 of socketiooxide the ".of" methods not works for dynamic namespace
        // i read this after implementing it below and else where in the code, but I;ve raised a PR
        // requesting the feature until then this code is commented !!

        let namespace_size = io.of(s.ns()).unwrap().sockets().unwrap().len();

        log_message!("DEBUG", "NAMESAPCE = {} OF SIZE = {}", s.ns(), namespace_size);
        log_message!("DEBUG", "AUTHENTICATION TOKEN RECIEVED ON THE NEW NAMESPACE WS TEST ROUTE: {}", &auth_token[1..]);

        let _ = s.emit("LOG", "PING");
        s.on(
            "JOIN", 
            |
            s: SocketRef, 
            Data::<JoinLeaveRequestData>(payload), 
            ack: AckSender| async move 
        {
            let _ = s.join(payload.group_id.clone());
            ack.send(format!("Joined the group 💯💯🧠 !! {:?}", s.rooms().unwrap()).as_str()).ok();
        });
        authenticate_test_clients(s, &auth_token[1..], namespace_size).await;
    });

    let _ = io.dyn_ns("/{client_id}", |s: SocketRef, io: SocketIo| async move {

        let auth_token = s.ns().to_string();

        // # RESOLVED
        // untill version 15 of socketiooxide the ".of" methods not works for dynamic namespace
        // i read this after implementing it below and else where in the code, but I;ve raised a PR
        // requesting the feature until then this code is commented !!

        let namespace_size = io.of(s.ns()).unwrap().sockets().unwrap().len();

        s.on(
            "INFO",
            |Data::<MyAuthData>(payload), ack: AckSender| async move {
                info_handler(payload, ack);
            },
        );
        s.on("MESSAGE", message_handler);
        s.on("LEAVE", leave_handler);
        s.on(
            "JOIN",
            |
                s: SocketRef, 
                Data::<JoinLeaveRequestData>(payload), 
                ack: AckSender| async move {
                join_handler(s, payload, ack);
            },
        );

        authenticate_clients(s, &auth_token[1..], namespace_size, db).await;
    });

}