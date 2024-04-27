use serde_json::json;

use socketioxide::extract::{
    AckSender, 
    Data, 
    SocketRef
};

use crate::map::{
    decrement_user_hits, 
    get_user_from_hash, 
    remove_user_from_hash
};

use crate::structs::{
    ClientMessage, 
    JoinLeaveRequestData, MyAuthData
};

use crate::util::devlog;

pub fn info_handler(message: MyAuthData, ack: AckSender) {
    if let Some(user) = get_user_from_hash(&message.token) {
        ack.send(json!({
            "connections": user.connections,
            "hits": user.hits,
            "status": "ok"
        })).ok();
    }
    else {
        let _ = ack.send(json!({
            "status": "error",
            "message": "User with given key does not exists on the server."
        }));
    }
}

pub fn join_handler(socket: SocketRef, message: JoinLeaveRequestData, ack: AckSender) {
    // println!("👀🤗🫡 Received Join Group Request for Room: {:?}", message);
    decrement_user_hits(&message.key);
    // let _ = socket.leave_all();
    devlog(&format!("Recived a join request from {:?}", message));
    let _ = socket.join(message.key+"_"+&message.group_id); // key_group_id = group
    ack.send("Joined the group !!").ok();
}

pub fn leave_handler(socket: SocketRef, room: JoinLeaveRequestData, ack: AckSender) {
    // println!("👀🤗🫡 Received Leave Group Request for Room: {:?}", room);
    decrement_user_hits(&room.key);
    devlog(&format!("Recived a leave request from {:?}", room));
    let _ = socket.leave(room.key+"_"+&room.group_id.clone());
    ack.send("Left the group !!").ok();
}

pub fn message_handler(socket: SocketRef, message: ClientMessage) {

    devlog(&format!("Recived a message from {:?}", message));

    // println!("👀🤗🫡 Received Message for Room: {:?}", message);
    // ack.send("Message Recived !!").ok();
    
    if let Some(user) = get_user_from_hash(&message.key) {
        if user.hits <= 0 {
            let _ = socket.emit("ERROR", "💥 Daily Limit Reached 🥹");
            let _ = socket.disconnect();
            remove_user_from_hash(&message.key);
            return;
        }
        decrement_user_hits(&message.key);
        let _ = socket
            .except(socket.id) // broadcast the message to everyone except the sender.
            .within(message.group_id)
            .broadcast()
            .emit(message.event_name, &message.message)
        ;
    }
}