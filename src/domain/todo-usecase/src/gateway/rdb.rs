use async_trait::async_trait;
use todo_model::ToDo;

use crate::error::Result;

#[async_trait]
pub trait DBService {
    async fn insert_todo(&self, todo: ToDo) -> Result<()>;
}
