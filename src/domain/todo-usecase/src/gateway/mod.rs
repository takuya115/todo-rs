use self::rdb::RdbService;

pub mod rdb;

pub trait Gateway {
    fn db_service(&self) -> &dyn RdbService;
}
