use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};

pub fn get_request_limit_from_plan_name(val: &str) -> i64 {
    match val {
        "Hobby" => 500,
        "Pro" => 1000000,
        "StartUp" => 5000000,
        _ => 0,
    }
}

pub fn get_connection_limit_from_plan_name(val: &str) -> i64 {
    match val {
        "Hobby" => 10,
        "Pro" => 500,
        "StartUp" => 2000,
        _ => 0,
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JoinLeaveRequestData {
    pub key: String,
    pub group_id: String,  
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserLimits {
    pub hits: i64,
    pub connections: i64,
    pub plan: String,
}

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

pub static mut CONNECTION_HASH: Mutex<Option<HashMap<String, UserLimits>>> = Mutex::new(None);

pub fn init_map() {
    let mut guarded_hash = unsafe { CONNECTION_HASH.lock().unwrap() };
    *guarded_hash = Some(HashMap::<String, UserLimits>::new());
}

pub fn get_user_hash_map() -> Result<String, serde_json::Error> {
    let guarded_hash = unsafe { CONNECTION_HASH.lock().unwrap() };
    serde_json::to_string(guarded_hash.as_ref().unwrap())
}

pub fn reset_all_users_values() {
    let mut guarded_hash = unsafe { CONNECTION_HASH.lock().unwrap() };
    if let Some(map) = guarded_hash.as_mut() {
        for (_, user) in map.iter_mut() {
            user.hits = get_request_limit_from_plan_name(&user.plan);
            user.connections = get_connection_limit_from_plan_name(&user.plan);
        }
    }
}

pub fn add_user_to_hash(key: String, user: UserLimits) {
    let mut guarded_hash = unsafe { CONNECTION_HASH.lock().unwrap() };
    if let Some(map) = guarded_hash.as_mut() {
        map.insert(key, user);
        println!("🤘🚀 Server Connection HashMap: `{}`", map.len());
    }
}

pub fn remove_user_from_hash(key: &str) {
    let mut guarded_hash = unsafe { CONNECTION_HASH.lock().unwrap() };
    if let Some(map) = guarded_hash.as_mut() {
        map.remove(key);
    }
}

pub fn decrement_user_hits(key: &str) {
    let mut guarded_hash = unsafe { CONNECTION_HASH.lock().unwrap() };
    if let Some(map) = guarded_hash.as_mut() {
        if let Some(user) = map.get_mut(key) {
            user.hits -= 1;
        }
    }
}

pub fn get_user_from_hash(key: &str) -> Option<UserLimits> {
    let guarded_hash = unsafe { CONNECTION_HASH.lock().unwrap() };
    if let Some(map) = guarded_hash.as_ref() {
        map.get(key).cloned()
    }
    else {
        None
    }
}

pub fn decrement_user_connections(key: &str) {
    let mut guarded_hash = unsafe { CONNECTION_HASH.lock().unwrap() };
    if let Some(map) = guarded_hash.as_mut() {
        if let Some(user) = map.get_mut(key) {
            user.connections -= 1;
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
