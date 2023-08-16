use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct ToDoId(Uuid);
impl ToDoId {
    fn new() -> Self {
        ToDoId(Uuid::new_v4())
    }
}

// todo domain model
#[derive(Debug)]
pub struct ToDo {
    pub id: ToDoId,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_done: bool,
}

#[test]
fn ok() {
    let todo = ToDo {
        id: ToDoId::new(),
        message: "test-massage".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        is_done: false,
    };
    println!("{todo:?}");
}
