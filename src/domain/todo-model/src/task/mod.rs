mod id;
pub use id::TaskId;
mod body;
pub use body::TaskBody;

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Task {
    pub id: TaskId,
    pub content: TaskBody,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub done: bool,
}
