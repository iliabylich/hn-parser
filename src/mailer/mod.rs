use crate::{config::Config, state::AppState};
use std::time::Duration;
use tokio::time::interval;

mod gmail;
pub(crate) use gmail::Gmail;

pub(crate) struct Mailer;

impl Mailer {
    pub(crate) async fn spawn(state: AppState) {
        // state.gmail.send_test_email().await;

        let mut interval = interval(Duration::from_secs(
            Config::global().send_email_once_every_seconds,
        ));

        tokio::task::spawn(async move {
            loop {
                interval.tick().await;
                Self::tick(state.clone()).await;
            }
        })
        .await
        .expect("Failed to spawn mailer task");
    }

    async fn tick(state: AppState) {
        let jobs = state.database.new_jobs().await;
        println!("Mailer tick, new jobs count = {}", jobs.len());
        if jobs.is_empty() {
            return;
        }

        let body = state.views.jobs_email(&jobs);
        state.gmail.send_html_email("New jobs from HN", body).await
    }
}
