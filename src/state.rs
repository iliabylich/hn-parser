use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{fixture::Fixture, job::Job, post::Post};

#[derive(Clone, Debug)]
pub(crate) struct AppState {
    last_seen_job_id: u32,
    current_post: Option<Post>,
    current_jobs: Vec<Job>,
}

const CACHE_FILE: &str = "/tmp/hnparser-last-seen-job-id";

impl AppState {
    pub(crate) fn new() -> Result<Arc<Mutex<Self>>> {
        let last_seen_job_id = match std::fs::read_to_string(CACHE_FILE) {
            Ok(content) => match content.trim().parse::<u32>() {
                Ok(id) => id,
                Err(e) => {
                    println!("failed to parse state file: {}", e);
                    0
                }
            },
            Err(e) => {
                println!("failed to read state file: {}", e);
                0
            }
        };

        Ok(Arc::new(Mutex::new(Self {
            last_seen_job_id,
            current_post: None,
            current_jobs: vec![],
        })))
    }

    pub(crate) fn get_last_seen_job_id(&self) -> u32 {
        self.last_seen_job_id
    }

    pub(crate) fn get_current_post(&self) -> Post {
        self.current_post.clone().unwrap_or_else(Post::fixture)
    }

    pub(crate) fn get_current_jobs(&self) -> Vec<Job> {
        if self.current_jobs.is_empty() {
            vec![Job::fixture(); 10]
        } else {
            self.current_jobs.clone()
        }
    }

    pub(crate) fn update(&mut self, post: Post, jobs: Vec<Job>) -> Result<()> {
        self.current_post = Some(post);
        self.current_jobs = jobs;
        self.last_seen_job_id = self
            .current_jobs
            .iter()
            .map(|job| job.id)
            .max()
            .unwrap_or(self.last_seen_job_id);

        std::fs::write(CACHE_FILE, self.last_seen_job_id.to_string())
            .context("failed to write state file")
    }
}
