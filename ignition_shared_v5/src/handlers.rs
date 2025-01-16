use serde_json::json;
use crate::log_message; 

use socketioxide::extract::{
    AckSender, 
    Data, 
    SocketRef
};

use crate::dashmap::{
    decrement_user_hits, 
    get_user_from_hash, 
    remove_user_from_hash
};

use crate::structs::{
    ClientMessage, 
    JoinLeaveRequestData, MyAuthData
};

pub fn info_handler(message: MyAuthData, ack: AckSender) {
    if let Some(user) = get_user_from_hash(&message.token) {
        ack.send(&json!({
            "connections": user.connections,
            "hits": user.hits,
            "status": "ok"
        })).ok();
    }
    else {
        let _ = ack.send(&json!({
            "status": "error"
        }));
    }
}

pub fn join_handler(socket: SocketRef, message: JoinLeaveRequestData, ack: AckSender) {
    decrement_user_hits(&message.key, None);
    let _ = socket.leave_all();
    log_message!("DEBUG", "Recived a join request from {:?}", message);
    let _ = socket.join(message.group_id.clone());
    ack.send("Joined the group 💯💯🧠 !!").ok();
}

pub fn leave_handler(socket: SocketRef, Data(room): Data<JoinLeaveRequestData>, ack: AckSender) {
    decrement_user_hits(&room.key, None);
    log_message!("DEBUG", "Recived a leave request from {:?}", room);
    let _ = socket.leave(room.group_id.clone());
    ack.send("Left the group !!").ok();
}

pub fn message_handler(socket: SocketRef, Data(message): Data<ClientMessage>) {

    log_message!("DEBUG", "Recived a message from {:?}", message);

    if let Some(user) = get_user_from_hash(&message.key) {

        if user.hits <= 0 {
            let _ = socket.emit("ERROR", "💥 Daily Limit Reached 🥹");
            let _ = socket.disconnect();
            remove_user_from_hash(&message.key);
            return;
        }

        decrement_user_hits(&message.key, None);

        let _ = socket
            .within(message.group_id)
            .broadcast()
            .emit(message.event_name, &message.message)
        ;
    }
}