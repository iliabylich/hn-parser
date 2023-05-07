use sqlx::sqlite::SqlitePool;

use crate::config::Config;

pub(crate) struct Database {
    pub(crate) pool: SqlitePool,
}

fn database_url() -> String {
    let config = Config::global();
    format!("sqlite:{}", config.database_path)
}

impl Database {
    pub(crate) async fn new() -> Self {
        let pool = SqlitePool::connect(&database_url())
            .await
            .expect("Failed to connect to sqlite");
        println!("Connected to sqlite");

        Self { pool }
    }
}
