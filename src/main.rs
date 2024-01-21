use std::sync::Arc;

use anyhow::Result;

mod app_error;
mod config;
mod fixture;
mod highlighter;
mod hn_client;
mod job;
mod mailer;
mod poll;
mod post;
mod state;
mod templates;
mod views;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    use crate::{config::Config, mailer::Mailer, poll::Poll, state::AppState, web::Web};

    let state = AppState::new()?;

    Config::setup()?;
    println!("Running with config {:?}", Config::global());

    Mailer::setup()?;

    tokio::try_join!(
        Poll::spawn(Arc::clone(&state)),
        Web::spawn(Arc::clone(&state))
    )?;

    Ok(())
}
