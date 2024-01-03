use todo_model::{Todo, TodoId};

use crate::error::Result;

use super::Interactor;

pub struct CreateTodoInput {
    pub content: String,
}

impl Interactor {
    pub async fn create_todo(input: CreateTodoInput) -> Result<Todo> {
        let _todo = Todo {
            id: TodoId::generate(),
            content: input.content,
            ..Default::default()
        };
        todo!()
    }
}
