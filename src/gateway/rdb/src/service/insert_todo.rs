use sea_orm::{ActiveModelTrait, Set};
use todo_model::task::{Task, TaskBody, TaskId};
use todo_usecase::{Error, Result};

use crate::entity::tasks;

use super::RdbServiceImpl;

impl RdbServiceImpl {
    pub(crate) async fn handle_create_task(&self, id: TaskId, content: TaskBody) -> Result<Task> {
        let conn = self.connect().await?;
        let task = tasks::ActiveModel {
            id: Set(id.into()),
            content: Set(content.to_string()),
            done: Set(false),
            ..Default::default()
        };
        let record = task
            .insert(&conn)
            .await
            .map_err(|err| Error::Unexpected(Box::new(err)))?;
        Ok(record.into())
    }
}
