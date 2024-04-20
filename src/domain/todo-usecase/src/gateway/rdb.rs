use async_trait::async_trait;
use todo_model::task::{Task, TaskBody, TaskId};

use crate::error::Result;

#[async_trait]
pub trait RdbService {
    async fn create_task(&self, id: TaskId, content: TaskBody) -> Result<Task>;
}
