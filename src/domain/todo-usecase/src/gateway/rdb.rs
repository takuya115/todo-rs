use async_trait::async_trait;
use todo_model::{Todo, TodoId};

use crate::error::Result;

#[async_trait]
pub trait RdbService {
    async fn create_todo(&self, id: TodoId, content: String) -> Result<Todo>;
}
