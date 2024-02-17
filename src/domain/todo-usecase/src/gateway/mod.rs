use self::rdb::RdbService;

pub mod rdb;

/// ビジネス・ドメインロジックへのアクセスを提供する
pub trait Gateway: Send + Sync {
    fn db_service(&self) -> &dyn RdbService;
}
