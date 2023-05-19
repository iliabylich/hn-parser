use serde::{Deserialize, Serialize};

use crate::database::Database;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub(crate) struct Post {
    pub(crate) hn_id: i64,
    pub(crate) name: String,
}

const CREATE_POSTS_TABLE_SQL: &str = r#"
    CREATE TABLE IF NOT EXISTS posts (
        hn_id INTEGER PRIMARY KEY,
        name TEXT NOT NULL
    )
"#;

impl Post {
    pub(crate) async fn create_table(database: &Database) {
        sqlx::query(CREATE_POSTS_TABLE_SQL)
            .execute(&database.pool)
            .await
            .expect("failed to create `posts` table");
        println!("Created `posts` table");
    }
}
