use chrono::{DateTime, Utc};
use uuid::Uuid;

// todo domain model
#[derive(Debug)]
pub struct ToDo {
    pub id: Uuid,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_done: bool,
}

#[test]
fn ok() {
    let todo = ToDo {
        id: Uuid::new_v4(),
        message: "test-massage".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        is_done: false,
    };
    println!("{todo:?}");
}
