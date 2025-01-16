use serde::{
    Deserialize, 
    Serialize
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserLimits {
    pub hits: usize,
    pub connections: usize,
    pub plan: String,
    // pub lru_pos: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JoinLeaveRequestData {
    pub key: String,
    pub group_id: String,  
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

#[derive(Deserialize, Serialize)]
pub struct UpgradeKey {
    pub key: String,
}