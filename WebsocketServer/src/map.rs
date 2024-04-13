use std::{
    collections::HashMap, 
    sync::Mutex
};

use crate::{
    structs::UserLimits, 
    util::{
        get_connection_limit_from_plan_name, 
        get_request_limit_from_plan_name
    }
};

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
    }
    println!("HashMap length: `{}`", guarded_hash.as_ref().unwrap().len());
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