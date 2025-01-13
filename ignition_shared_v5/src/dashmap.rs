/// lazy_static! is a Rust macro provided by the lazy_static crate that allows you to define global static variables 
/// that are initialized lazily at runtime. These variables are initialized the first time they are accessed, and the 
/// initialization happens only once, ensuring thread safety. 
/// 
/// In Rust, regular static variables are limited to simple 
/// types that implement the Sync trait (e.g., integers, strings, or arrays with fixed sizes). 
/// You cannot directly use complex types like HashMap, Vec, or DashMap in static variables because their 
/// initialization may require runtime logic. 
/// 
/// With lazy_static!, you can initialize complex static variables at runtime in a thread-safe way.
/// 
/// LazyLock is a new simpler version to work with static complex types 💯💪
/// 
/// 
/// 
use std::collections::HashMap;
use dashmap::DashMap;
use std::sync::LazyLock;
use crate::{
    structs::UserLimits, 
    util::get_request_limit_from_plan_name
};

pub static CONNECTION_DASH: LazyLock<DashMap<String, UserLimits>> = LazyLock::new(|| DashMap::new());

pub fn init_map() {
    CONNECTION_DASH.clear();
}

/// to get the entire map of users via `/users` endpoint
pub fn get_user_hash_map() -> Result<String, serde_json::Error> {
    let mut map: HashMap<String, UserLimits> = HashMap::new();
    for user in CONNECTION_DASH.iter() {
        map.insert(user.key().clone(), user.value().clone());
    }
    serde_json::to_string(&map)
}

/// invoked by a lambda function via `/reset` endpoint each day at UTC midnight
pub fn reset_all_users_values() {
    for mut user_entries in CONNECTION_DASH.iter_mut() {
        user_entries.connections = 0;
        user_entries.hits = get_request_limit_from_plan_name(&user_entries.plan)
    }
}

/// to add a new user via first time authentication
pub fn add_user_to_hash(key: String, user: UserLimits) {
    CONNECTION_DASH.insert(key, user);
    println!("HashMap length: '{}'", CONNECTION_DASH.len());
}

/// Remove a user from the hashmap
pub fn remove_user_from_hash(key: &str) {
    CONNECTION_DASH.remove(key);
}

/// Decrement hits for a specific user
pub fn decrement_user_hits(key: &str) {
    if let Some(mut user) = CONNECTION_DASH.get_mut(key) {
        user.hits -= 1;
    }
}

/// Retrieve a user from the hashmap
pub fn get_user_from_hash(key: &str) -> Option<UserLimits> {
    CONNECTION_DASH.get(key).map(|user| user.clone())
}

/// Increment the connection count for a user
pub fn increment_user_connections(key: &str) {
    if let Some(mut user) = CONNECTION_DASH.get_mut(key) {
        user.connections += 1;
    }
}