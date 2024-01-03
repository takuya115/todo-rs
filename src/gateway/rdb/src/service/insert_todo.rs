use sea_orm::{ActiveModelTrait, Set};
use todo_model::{Todo, TodoId};
use todo_usecase::error::{Error, Result};

use crate::entity::todo_table;

use super::RdbServiceImpl;

impl RdbServiceImpl {
    pub(crate) async fn handle_insert_todo(&self, id: TodoId, message: String) -> Result<Todo> {
        let conn = self.connect().await?;
        let todo_entity = todo_table::ActiveModel {
            id: Set(id.into()),
            message: Set(message),
            is_done: Set(false),
            ..Default::default()
        };
        let record = todo_entity
            .insert(&conn)
            .await
            .map_err(|err| Error::Unknown(Box::new(err)))?;
        Ok(record.into())
    }
}
