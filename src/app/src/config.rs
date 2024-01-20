use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_host: String,
    pub db_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            server_host: env::var("SERVER_HOST").unwrap(),
            db_url: env::var("DATABASE_URL").unwrap(),
        }
    }
}
