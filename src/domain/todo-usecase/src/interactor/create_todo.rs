use todo_model::task::{Task, TaskBody, TaskId};

use crate::error::Result;

use super::Interactor;

pub struct CreateTodoInput {
    pub content: TaskBody,
}

impl Interactor {
    pub async fn create_todo(&self, input: CreateTodoInput) -> Result<Task> {
        self.gateway
            .db_service()
            .create_todo(TaskId::generate(), input.content)
            .await
    }
}
