use rdb::service::RdbServiceImpl;
use todo_usecase::gateway::{rdb::RdbService, Gateway};

use crate::config::Config;

pub struct GatewayImpl {
    pub db_service: RdbServiceImpl,
}

impl GatewayImpl {
    pub fn build(config: &Config) -> Self {
        Self {
            db_service: RdbServiceImpl {
                db_url: config.db_url.clone(),
            },
        }
    }
}

impl Gateway for GatewayImpl {
    fn db_service(&self) -> &dyn RdbService {
        &self.db_service
    }
}
