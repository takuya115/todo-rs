use self::rdb::DBService;

pub mod rdb;

pub trait Gateway {
    fn db_service(&self) -> dyn DBService;
}
