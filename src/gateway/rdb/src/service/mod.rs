mod insert_todo;
use async_trait::async_trait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use todo_model::{Todo, TodoId};
use todo_usecase::{
    error::{Error, Result},
    gateway::rdb::RdbService,
};

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
    async fn create_todo(&self, id: TodoId, content: String) -> Result<Todo> {
        todo!()
    }
}
