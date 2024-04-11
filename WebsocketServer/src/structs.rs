use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use socketioxide::extract::SocketRef;

#[derive(Eq, Hash, PartialEq, Deserialize, Debug)]
pub struct MyAuthData {
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ClientMessage {
    pub group_id: String,
    pub event_name: String,
    pub message: String,
    pub key: String,
}

pub static mut CONNECTION_HASH: Option<HashMap<String, SocketRef>> = None;

pub fn init_map() {
    unsafe { CONNECTION_HASH = Some(HashMap::<String, SocketRef>::new()) };
}

pub fn insert_ws(key: String, ws_ref: SocketRef) {
    unsafe {
        if let Some(map) = CONNECTION_HASH.as_mut() {
            map.insert(key, ws_ref);
        }
        println!("🤘🚀 Server Connection HashMap: `{}`", CONNECTION_HASH.clone().unwrap().len());
    }
}

pub fn get_ws(key: &String) -> Option<SocketRef> {
    unsafe {
        if let Some(map) = CONNECTION_HASH.as_ref() {
            map.get(key).cloned()
        } else {
            None
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub key: String,
    pub group_id: String,
    pub event_name: String,
    pub message: String
}

pub struct PollingCounters {
    pub null_counter: i32,
    pub full_counter: i32
}

impl PollingCounters {
    pub fn reset(&mut self) {
        self.null_counter = 0;
        self.full_counter = 0;
    }

    pub fn incre_null_counter(&mut self) {
        self.null_counter = self.null_counter + 1;
    }

    pub fn incre_full_counter(&mut self) {
        self.full_counter = self.full_counter + 1;
    }

    pub fn reset_null_counter(&mut self) {
        self.null_counter = 0;
    }

    pub fn reset_full_counter(&mut self) {  
        self.full_counter = 0;
    }
}
