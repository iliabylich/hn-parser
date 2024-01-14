use anyhow::{Context, Result};
use std::time::Duration;
use tokio::time::interval;

use crate::{config::Config, state::AppState, views::Views};

mod gmail;
pub(crate) use gmail::Gmail;

pub(crate) struct Mailer;

impl Mailer {
    pub(crate) async fn spawn(state: AppState) -> Result<()> {
        // state.gmail.send_test_email().await;

        let mut interval = interval(Duration::from_secs(
            Config::global()?.send_email_once_every_seconds,
        ));

        tokio::task::spawn(async move {
            loop {
                interval.tick().await;
                match Self::tick(state.clone()).await {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Mailer tick failed: {:?}", e);
                    }
                }
            }
        })
        .await
        .context("Failed to spawn mailer task")
    }

    async fn tick(state: AppState) -> Result<()> {
        let jobs = state.database.new_jobs().await?;
        println!("Mailer tick, new jobs count = {}", jobs.len());
        if jobs.is_empty() {
            return Ok(());
        }

        let body = Views::jobs_email(&jobs)?;
        state
            .gmail
            .send_html_email("New jobs from HN", body)
            .await
            .context("failed to send an email")
    }
}
