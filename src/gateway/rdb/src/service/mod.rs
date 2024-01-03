mod insert_todo;
use async_trait::async_trait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use todo_model::{Text, Todo, TodoId};
use todo_usecase::{
    error::{Error, Result},
    gateway::rdb::RdbService,
};

use crate::entity::todo_table;

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
            .map_err(|err| Error::Unknown(Box::new(err)))
    }
}

#[async_trait]
impl RdbService for RdbServiceImpl {
    async fn create_todo(&self, id: TodoId, content: Text) -> Result<Todo> {
        self.handle_create_todo(id, content).await
    }
}

impl From<todo_table::Model> for Todo {
    fn from(value: todo_table::Model) -> Self {
        Self {
            id: value.id.into(),
            content: Text::from_str_unchecked(&value.content),
            created_at: value.created_at,
            updated_at: value.updated_at,
            done: value.done,
        }
    }
}
