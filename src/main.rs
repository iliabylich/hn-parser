use crate::{
    config::Config, mailer_task::MailerTask, scraper_task::ScraperTask, state_task::StateTask,
    web::Web,
};
use anyhow::Result;

mod app_error;
mod config;
mod highlighter;
mod mailer;
mod mailer_task;
mod non_empty_vec;
mod scraper;
mod scraper_task;
mod state;
mod state_task;
mod templates;
mod views;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let config = Config::read().await?;
    log::info!("Running with config {config:?}");

    let (scraper, mut scraper_rx) = ScraperTask::spawn(&config);
    let (state, state_ctl, mut state_rx) = StateTask::spawn(&config)?;
    let (mailer, mailer_ctl) = MailerTask::spawn(&config);
    let web = Web::spawn(state_ctl.clone(), &config).await?;

    let router = tokio::spawn(async move {
        loop {
            tokio::select! {
                Some((root, items)) = scraper_rx.recv() => {
                    state_ctl.set(root, items).await;
                }

                Some(new_jobs) = state_rx.recv() => {
                    mailer_ctl.send(new_jobs).await;
                }
            }
        }
    });

    tokio::try_join!(scraper, state, mailer, router, web)?;

    Ok(())
}
