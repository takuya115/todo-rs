use todo_model::task::{Task, TaskBody, TaskId};

use crate::error::Result;

use super::Interactor;

pub struct CreateTaskInput {
    pub task: TaskBody,
}

impl Interactor {
    pub async fn create_task(&self, input: CreateTaskInput) -> Result<Task> {
        self.gateway
            .db_service()
            .create_task(TaskId::generate(), input.task)
            .await
    }
}
