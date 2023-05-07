use crate::database::Database;

pub(crate) struct Schema;

const CREATE_POSTS_TABLE_SQL: &str = r#"
    CREATE TABLE IF NOT EXISTS posts (
        hn_id INTEGER PRIMARY KEY,
        name TEXT NOT NULL
    )
"#;

const CREATE_JOBS_TABLE_SQL: &str = r#"
    CREATE TABLE IF NOT EXISTS jobs (
        hn_id INTEGER PRIMARY KEY,
        text TEXT NOT NULL,
        by TEXT NOT NULL,
        post_hn_id INTEGER NOT NULL
    )
"#;

impl Schema {
    pub(crate) async fn apply(database: &Database) {
        sqlx::query(CREATE_POSTS_TABLE_SQL)
            .execute(&database.pool)
            .await
            .expect("failed to create `posts` table");
        println!("Created `posts` table");

        sqlx::query(CREATE_JOBS_TABLE_SQL)
            .execute(&database.pool)
            .await
            .expect("failed to create `jobs` table");
        println!("Created `jobs` table");
    }
}
