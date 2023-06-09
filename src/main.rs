mod config;
mod database;
mod fixture;
mod hn_client;
mod job;
mod keyword;
mod liquid;
mod mailer;
mod poll;
mod post;
mod state;
mod template;
mod views;
mod web;

#[tokio::main]
async fn main() {
    use crate::{
        config::Config,
        database::Database,
        job::Job,
        mailer::{Gmail, Mailer},
        poll::Poll,
        post::Post,
        state::AppState,
        views::Views,
        web::Web,
    };

    Config::load();
    println!("Running with config {:?}", Config::global());

    let db = Database::new().await;
    Post::create_table(&db).await;
    Job::create_table(&db).await;

    let views = Views::new();

    let gmail = Gmail::from_global_config();

    let state = AppState::new(db, views, gmail);

    tokio::join!(
        Poll::spawn(state.clone()),
        Web::spawn(state.clone()),
        Mailer::spawn(state.clone())
    );
}
