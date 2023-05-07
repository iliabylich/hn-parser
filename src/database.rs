use sqlx::sqlite::SqlitePool;

use crate::{config::Config, job::Job, post::Post};

#[derive(Clone, Debug)]
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

    pub(crate) async fn create_post_if_missing(&self, post: &Post) {
        sqlx::query(
            r#"
                INSERT OR IGNORE INTO posts (hn_id, name)
                VALUES (?, ?)
            "#,
        )
        .bind(post.hn_id)
        .bind(&post.name)
        .execute(&self.pool)
        .await
        .expect("Failed to create post");
    }

    pub(crate) async fn all_posts(&self) -> Vec<Post> {
        sqlx::query_as(r#"SELECT hn_id, name FROM posts"#)
            .fetch_all(&self.pool)
            .await
            .expect("Failed to fetch posts")
    }

    pub(crate) async fn last_post(&self) -> Option<Post> {
        sqlx::query_as(r#"SELECT hn_id, name FROM posts ORDER BY hn_id DESC LIMIT 1"#)
            .fetch_optional(&self.pool)
            .await
            .expect("Failed to fetch last post")
    }

    pub(crate) async fn create_job(&self, job: &Job) -> bool {
        let rows_affected = sqlx::query(
            r#"
                INSERT OR IGNORE INTO jobs (hn_id, text, by, post_hn_id)
                VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(job.hn_id)
        .bind(&job.text)
        .bind(&job.by)
        .bind(job.post_hn_id)
        .execute(&self.pool)
        .await
        .expect("Failed to create job")
        .rows_affected();

        rows_affected == 1
    }

    pub(crate) async fn max_job_id(&self) -> u64 {
        let max_id: i64 = sqlx::query_scalar(r#"SELECT MAX(hn_id) FROM jobs"#)
            .fetch_one(&self.pool)
            .await
            .unwrap_or_default();
        max_id as u64
    }

    pub(crate) async fn list_jobs(&self, post_hn_id: i64) -> Vec<Job> {
        sqlx::query_as(
            r#"
                SELECT hn_id, text, by, post_hn_id
                FROM jobs
                WHERE post_hn_id = ?
            "#,
        )
        .bind(post_hn_id)
        .fetch_all(&self.pool)
        .await
        .expect("Failed to fetch jobs")
    }
}
