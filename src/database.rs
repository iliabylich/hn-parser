use anyhow::{Context, Result};
use sqlx::sqlite::SqlitePool;

use crate::{config::Config, job::Job, post::Post};

#[derive(Clone, Debug)]
pub(crate) struct Database {
    pub(crate) pool: SqlitePool,
}

fn database_url() -> Result<String> {
    let config = Config::global()?;
    Ok(format!("sqlite:{}", config.database_path))
}

impl Database {
    pub(crate) async fn new() -> Result<Self> {
        let pool = SqlitePool::connect(&database_url()?)
            .await
            .context("Failed to connect to sqlite")?;
        println!("Connected to sqlite");

        Ok(Self { pool })
    }

    pub(crate) async fn load_schema(&self) -> Result<()> {
        sqlx::query(include_str!("../schema.sql"))
            .execute(&self.pool)
            .await
            .context("failed to load schema")?;
        println!("Schema loaded");
        Ok(())
    }

    pub(crate) async fn create_post_if_missing(&self, post: Post) -> Result<()> {
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
        .context("Failed to create post")?;
        Ok(())
    }

    pub(crate) async fn last_post(&self) -> Result<Option<Post>> {
        sqlx::query_as(r#"SELECT hn_id, name FROM posts ORDER BY hn_id DESC LIMIT 1"#)
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch last post")
    }

    pub(crate) async fn create_job(&self, job: &Job) -> Result<bool> {
        let rows_affected = sqlx::query(
            r#"
                INSERT OR IGNORE INTO jobs (hn_id, text, by, post_hn_id, time, interesting, email_sent)
                VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(job.hn_id)
        .bind(&job.text)
        .bind(&job.by)
        .bind(job.post_hn_id)
        .bind(job.time)
        .bind(job.interesting)
        .bind(job.email_sent)
        .execute(&self.pool)
        .await
        .context("Failed to create job")?
        .rows_affected();

        Ok(rows_affected == 1)
    }

    pub(crate) async fn max_job_id(&self) -> u32 {
        sqlx::query_scalar(r#"SELECT MAX(hn_id) FROM jobs"#)
            .fetch_one(&self.pool)
            .await
            .unwrap_or_default()
    }

    pub(crate) async fn list_jobs(&self, post_hn_id: u32) -> Result<Option<Vec<Job>>> {
        let jobs = sqlx::query_as(
            r#"
                SELECT hn_id, text, by, post_hn_id, time, interesting, email_sent
                FROM jobs
                WHERE post_hn_id = ? AND interesting = 1
            "#,
        )
        .bind(post_hn_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch jobs")?;

        if jobs.is_empty() {
            Ok(None)
        } else {
            Ok(Some(jobs))
        }
    }

    pub(crate) async fn new_jobs(&self) -> Result<Vec<Job>> {
        let jobs = sqlx::query_as(
            r#"
                SELECT hn_id, text, by, post_hn_id, time, interesting, email_sent
                FROM jobs
                WHERE interesting = 1 AND email_sent = 0
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch new jobs")?;

        sqlx::query("UPDATE jobs SET email_sent = 1")
            .execute(&self.pool)
            .await
            .context("Failed to update email_sent")?;

        Ok(jobs)
    }
}
