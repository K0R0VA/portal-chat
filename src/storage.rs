use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Pool};
use deadpool_postgres::tokio_postgres::NoTls;
use deadpool_postgres::config::ConfigError;

pub fn get_pool() -> Result<Pool, ConfigError> {
    let mut cfg = Config::new();
    cfg.host = Some("localhost".to_string());
    cfg.port = Some(5432);
    cfg.dbname = Some("chat".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("admin".to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    cfg.create_pool(NoTls)
}