use todo_model::{Text, Todo, TodoId};

use crate::error::Result;

use super::Interactor;

pub struct CreateTodoInput {
    pub content: Text,
}

impl Interactor {
    pub async fn create_todo(&self, input: CreateTodoInput) -> Result<Todo> {
        self.gateway
            .db_service()
            .create_todo(TodoId::generate(), input.content)
            .await
    }
}
