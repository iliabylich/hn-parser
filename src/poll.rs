use anyhow::{Context, Result};
use std::{sync::Arc, time::Duration};
use tokio::{
    sync::{Mutex, OnceCell},
    time::{interval, Interval},
};

use crate::{
    config::Config, hn_client::HnClient, job::Job, mailer::Mailer, post::Post, state::AppState,
};

#[derive(Debug)]
pub(crate) struct Poll {
    state: Arc<Mutex<AppState>>,
}

static POLL: OnceCell<Poll> = OnceCell::const_new();

fn interval_from_config() -> Result<Option<Interval>> {
    let config = Config::global()?;
    if config.poll_interval_in_seconds == 0 {
        println!("Polling is disabled");
        return Ok(None);
    }
    Ok(Some(interval(Duration::from_secs(
        config.poll_interval_in_seconds,
    ))))
}

impl Poll {
    fn setup(state: Arc<Mutex<AppState>>) -> Result<()> {
        let poll = Self { state };
        POLL.set(poll).context("Failed to set poll")?;
        Ok(())
    }

    fn global() -> Result<&'static Self> {
        POLL.get().context("global poll is not set")
    }

    pub(crate) async fn spawn(state: Arc<Mutex<AppState>>) -> Result<()> {
        Self::setup(state)?;

        let mut interval = if let Some(interval) = interval_from_config()? {
            interval
        } else {
            return Ok(());
        };

        let this = Self::global().context("global poll is not set")?;

        tokio::task::spawn(async move {
            loop {
                interval.tick().await;

                match this.tick().await {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Poll tick failed: {:?}", e);
                    }
                }
            }
        })
        .await
        .context("Failed to spawn poll task")
    }

    async fn tick(&self) -> Result<()> {
        let post: Post = HnClient::get_latest_post().await?.try_into()?;
        println!("Latest post: {} / {:?}", post.id, post.title);

        let last_seen_job_id = { self.state.lock().await.get_last_seen_job_id() };
        println!("Max job id: {:?}", last_seen_job_id);

        let config = Config::global().context("Failed to load config")?;

        let all_jobs = HnClient::get_jobs_under(post.id)
            .await?
            .into_iter()
            .filter_map(|item| Job::try_from(item).ok())
            .filter(|job| config.highlighter.can_highlight(&job.text))
            .collect::<Vec<_>>();

        let new_jobs = all_jobs
            .iter()
            .filter(|job| job.id > last_seen_job_id)
            .cloned()
            .collect::<Vec<_>>();
        println!(
            "Sync completed, found {} jobs, {} are new",
            all_jobs.len(),
            new_jobs.len()
        );

        Mailer::send_jobs_email(&new_jobs).await?;

        {
            let mut state = self.state.lock().await;
            state.update(post, all_jobs)?;
        }

        Ok(())
    }
}
