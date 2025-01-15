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
use std::{collections::{HashMap, VecDeque}, sync::Mutex};
use dashmap::DashMap;
use std::sync::LazyLock;
use crate::{
    log_message, structs::UserLimits, util::get_request_limit_from_plan_name
};

const MAX_LRU_CACHE_SIZE: usize = 100;

pub static CONNECTION_DASH: LazyLock<DashMap<String, UserLimits>> = LazyLock::new(|| DashMap::new());
pub static LRU_CACHE: LazyLock<Mutex<VecDeque<String>>> = LazyLock::new(
    || Mutex::new(
        VecDeque::with_capacity(MAX_LRU_CACHE_SIZE)
    )
);

/// to add a new user via first time authentication
pub fn add_user_to_hash(key: String, mut user: UserLimits) {

    log_message!("DEBUG", "ADDING USER TO HAHS AND CACHE !");

    let mut cache = LRU_CACHE.lock().unwrap();
    cache.push_back(key.clone());

    // ^ store a pointer, which points to the key in the lru for faster lookups !!
    user.lru_pos = cache.len() - 1;

    CONNECTION_DASH.insert(key, user);

    if cache.len() >= MAX_LRU_CACHE_SIZE {
        if let Some(least_used_key) = cache.pop_front() {
            CONNECTION_DASH.remove(&least_used_key);
        }
    }

    println!("HashMap length: '{}'", CONNECTION_DASH.len());
}
    
pub fn validate_lru(key: &str) {

    log_message!("DEBUG", "VALIDATING CACHE FOR KEY: {}", key);

    if let Some(mut user) = CONNECTION_DASH.get_mut(key) {

        let mut cache = LRU_CACHE.lock().unwrap();
        let (curr_user_index, ahead_user_index) = (user.lru_pos, user.lru_pos - 1);

        // move the pointer of of current user ahaed
        user.lru_pos = ahead_user_index;

        // move the pointer of the user ahead of the current user back, and also swap them in the cache
        if ahead_user_index < cache.len() {
            if let Some(ahead_user_key) = cache.get(ahead_user_index) {
                if let Some(mut ahead_user) = CONNECTION_DASH.get_mut(ahead_user_key) {
                    ahead_user.lru_pos = curr_user_index;
                }
            }
            cache.swap(curr_user_index, ahead_user_index);
        }
    }
}

/// Decrement hits for a specific user
pub fn decrement_user_hits(key: &str) {

    log_message!("DEBUG", "USER HITS DECREMENTED, KEY: {}", key);

    if let Some(mut user) = CONNECTION_DASH.get_mut(key) {
        user.hits -= 1;
        validate_lru(key);
    }

}

/// Increment the connection count for a user
pub fn increment_user_connections(key: &str) {
    if let Some(mut user) = CONNECTION_DASH.get_mut(key) {
        user.connections += 1;
        validate_lru(key);
    }
}

/// Retrieve a user from the hashmap
pub fn get_user_from_hash(key: &str) -> Option<UserLimits> {
    CONNECTION_DASH.get(key).map(|user| user.clone())
}

/// Remove a user from the hashmap
pub fn remove_user_from_hash(key: &str) {
    CONNECTION_DASH.remove(key);
    // ^ We only remove the user from the DashMap it would automatically 
    // ^ be discarded from the cache after some time !!
    // let lru_pos_of_user: usize;
    // if let Some(user) = CONNECTION_DASH.remove(key) {
    //     lru_pos_of_user = user.1.lru_pos;
    //     let mut cache = LRU_CACHE.lock().unwrap();
    //     cache.remove(lru_pos_of_user);
    // };
}

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



// `.position()` : Searches for an element in an iterator, returning its index.
// if let Some(key_position) = cache.iter().position(|k| k == &key) {
//     if key_position > 0 {
//         cache.swap(key_position, key_position - 1);
//     }
// }


// skiped iterating through the entire VecDeque as we stored the lru_pos pointer in the DashMap !!

    // if let Some(pos) = cache.iter().position(|k| k == key) {
    //     cache.remove(pos);
    // }
