mod id;
pub use id::*;
mod error;
pub use error::*;

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Todo {
    pub id: TodoId,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub done: bool,
}

impl Default for Todo {
    fn default() -> Self {
        Self {
            id: TodoId::generate(),
            content: String::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            done: false,
        }
    }
}
