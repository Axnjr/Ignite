use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MyAuthData {
    pub token: String,
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
