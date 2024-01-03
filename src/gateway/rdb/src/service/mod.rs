use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use todo_usecase::error::{Error, Result};

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
