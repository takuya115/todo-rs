mod insert_todo;
use async_trait::async_trait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use todo_model::task::{Task, TaskBody, TaskId};
use todo_usecase::{gateway::rdb::RdbService, Error, Result};

use crate::entity::tasks;

/// RDBの具象
pub struct RdbServiceImpl {
    pub db_url: String,
}

impl RdbServiceImpl {
    /// DBと接続する
    async fn connect(&self) -> Result<DatabaseConnection> {
        let mut opt = ConnectOptions::new(&self.db_url);
        opt.max_connections(10).min_connections(2);
        Database::connect(opt)
            .await
            .map_err(|err| Error::Unexpected(Box::new(err)))
    }
}

#[async_trait]
impl RdbService for RdbServiceImpl {
    async fn create_task(&self, id: TaskId, content: TaskBody) -> Result<Task> {
        self.handle_create_task(id, content).await
    }
}

impl From<tasks::Model> for Task {
    fn from(value: tasks::Model) -> Self {
        Self {
            id: value.id.into(),
            content: TaskBody::from_str_unchecked(&value.content),
            created_at: value.created_at,
            updated_at: value.updated_at,
            done: value.done,
        }
    }
}
