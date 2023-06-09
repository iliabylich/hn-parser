use serde::{Deserialize, Serialize};

use crate::{config::Config, database::Database};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Job {
    pub(crate) hn_id: i64,
    pub(crate) text: String,
    pub(crate) by: String,
    pub(crate) post_hn_id: i64,
    pub(crate) time: i64,
    pub(crate) interesting: bool,
    pub(crate) email_sent: bool,
}

const CREATE_JOBS_TABLE_SQL: &str = r#"
    CREATE TABLE IF NOT EXISTS jobs (
        hn_id INTEGER PRIMARY KEY,
        text TEXT NOT NULL,
        by TEXT NOT NULL,
        post_hn_id INTEGER NOT NULL,
        time INTEGER NOT NULL,
        interesting BOOLEAN NOT NULL,
        email_sent BOOLEAN NOT NULL DEFAULT FALSE
    )
"#;

impl Job {
    pub(crate) async fn create_table(database: &Database) {
        sqlx::query(CREATE_JOBS_TABLE_SQL)
            .execute(&database.pool)
            .await
            .expect("failed to create `jobs` table");
        println!("Created `jobs` table");
    }

    pub(crate) fn has_keywords(&self) -> bool {
        Config::global()
            .keyword_regexes
            .iter()
            .any(|keyword| keyword.is_match(&self.text))
    }

    pub(crate) fn highlight_keywords<F>(&mut self, f: F)
    where
        F: Fn(&str) -> String,
    {
        Config::global()
            .keyword_regexes
            .iter()
            .for_each(|keyword| self.text = keyword.replace_all(&self.text, |capture| f(capture)));
    }
}
