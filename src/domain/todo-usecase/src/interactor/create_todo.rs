use chrono::Utc;
use todo_model::{ToDo, ToDoId};

use crate::error::Result;

use super::Interactor;

pub struct CreateTodoInput {
    pub content: String,
}

impl Interactor {
    pub async fn create_todo(input: CreateTodoInput) -> Result<ToDo> {
        let now = Utc::now();
        let _todo = ToDo {
            id: ToDoId::generate(),
            content: input.content,
            created_at: now,
            updated_at: now,
            done: false,
        };
        todo!()
    }
}
