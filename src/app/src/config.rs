use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_host: String,
    pub db_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            server_host: env::var("SERVER_HOST").expect("`SERVER_HOST` is not found."),
            db_url: env::var("DATABASE_URL").expect("`DATABASE_URL` is not found."),
        }
    }
}
