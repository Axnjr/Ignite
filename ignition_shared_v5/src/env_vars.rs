use dotenv::dotenv;
use std::env;
use crate::logging::log_and_panic;

pub struct Config {
    pub db_url: String,
    pub max_connections: u32,
    pub port: u16,
    pub mode: String
}

impl Config {
    fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok(); // Load .env variables
        Ok(Self {
            db_url: env::var("DB_URL")?,
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
            max_connections: env::var("MAX_DB_CONNECTIONS")?.parse().unwrap(),
            mode: env::var("MODE")?
        })
    }
}

use std::sync::LazyLock;

pub static SERVER_CONFIG: LazyLock<Config> = LazyLock::new(|| {
    Config::from_env().unwrap_or_else(|_| log_and_panic("Error reading enviroment variables !!"))
});