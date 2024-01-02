mod id;
pub use id::*;
mod error;
pub use error::*;

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct ToDo {
    pub id: ToDoId,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub done: bool,
}
