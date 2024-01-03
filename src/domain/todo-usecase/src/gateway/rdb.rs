use async_trait::async_trait;
use todo_model::Todo;

use crate::error::Result;

#[async_trait]
pub trait DBService {
    async fn insert_todo(&self, todo: Todo) -> Result<()>;
}
