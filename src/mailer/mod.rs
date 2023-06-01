use crate::state::AppState;
use std::time::Duration;
use tokio::time::interval;

pub(crate) mod gmail;

pub(crate) struct Mailer;

impl Mailer {
    pub(crate) async fn spawn(state: AppState) {
        let gmail = gmail::Gmail::from_global_config();
        gmail.send_test_email().await;

        let mut interval = interval(Duration::from_secs(5));

        tokio::task::spawn(async move {
            loop {
                interval.tick().await;
                Self::tick(state.clone(), gmail.clone()).await;
            }
        })
        .await
        .unwrap();
    }

    async fn tick(state: AppState, gmail: gmail::Gmail) {
        let jobs = state.database.new_jobs().await;
        println!("Mailer tick, new jobs count = {}", jobs.len());
        if jobs.is_empty() {
            return;
        }

        let body = state.views.jobs_email(&jobs);
        gmail.send_html_email("New jobs from HN", body).await
    }
}
