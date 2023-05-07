use crate::database::Database;

pub(crate) struct Schema;

const CREATE_POSTS_TABLE_SQL: &str = r#"
    CREATE TABLE IF NOT EXISTS posts (
        id INTEGER PRIMARY KEY,
        year INTEGER NOT NULL,
        month INTEGER NOT NULL,
        hn_id INTEGER NOT NULL
    )
"#;

const CREATE_JOBS_TABLE_SQL: &str = r#"
    CREATE TABLE IF NOT EXISTS jobs (
        id INTEGER PRIMARY KEY,
        hn_id INTEGER NOT NULL,
        text TEXT NOT NULL
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
