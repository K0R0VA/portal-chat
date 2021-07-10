use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Pool};
use tokio_postgres::NoTls;

pub fn get_config() -> Config {
    let mut cfg = Config::new();
    cfg.host = Some("localhost".to_string());
    cfg.port = Some(5432);
    cfg.dbname = Some("portal-chat".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("admin".to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    cfg
}

#[derive(Clone)]
pub struct Storage {
    pub(crate) pool: Pool,
}

impl Default for Storage {
    fn default() -> Self {
        Storage { pool: get_config().create_pool(NoTls).unwrap() }
    }
}