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
    JoinLeaveRequestData
};

pub fn join_handler(socket: SocketRef, message: JoinLeaveRequestData, ack: AckSender) {
    // println!("👀🤗🫡 Received Join Group Request for Room: {:?}", message);
    decrement_user_hits(&message.key);
    // let _ = socket.leave_all();
    let _ = socket.join(message.group_id.clone());
    ack.send("Joined the group !!").ok();
}

pub fn leave_handler(socket: SocketRef, Data(room): Data<JoinLeaveRequestData>, ack: AckSender) {
    // println!("👀🤗🫡 Received Leave Group Request for Room: {:?}", room);
    decrement_user_hits(&room.key);
    let _ = socket.leave(room.group_id.clone());
    ack.send("Left the group !!").ok();
}

pub fn message_handler(socket: SocketRef, Data(message): Data<ClientMessage>, ack: AckSender) {
    if let Some(user) = get_user_from_hash(&message.key) {
        if user.hits <= 0 {
            let _ = socket.emit("ERROR", "💥 Daily Limit Reached 🥹");
            let _ = socket.disconnect();
            remove_user_from_hash(&message.key);
            return;
        }
        decrement_user_hits(&message.key);
        let _ = socket
            .within(message.group_id)
            .broadcast()
            .emit(message.event_name, &message.message);
    }
}