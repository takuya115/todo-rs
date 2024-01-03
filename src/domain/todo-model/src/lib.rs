mod id;
use std::str::FromStr;

pub use id::*;
mod error;
pub use error::*;
mod text;
pub use text::*;

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Todo {
    pub id: TodoId,
    pub content: Text,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub done: bool,
}

impl Default for Todo {
    fn default() -> Self {
        Self {
            id: TodoId::generate(),
            content: Text::from_str("sample text").unwrap(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            done: false,
        }
    }
}
