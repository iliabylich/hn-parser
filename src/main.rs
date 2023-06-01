mod config;
mod database;
mod fixture;
mod hn_client;
mod job;
mod keyword;
mod liquid;
mod poll;
mod post;
mod state;
mod template;
mod views;
mod web;

#[tokio::main]
async fn main() {
    use crate::{
        config::Config, database::Database, job::Job, poll::Poll, post::Post, state::AppState,
        views::Views, web::Web,
    };

    Config::load();
    println!("Running with config {:?}", Config::global());

    let db = Database::new().await;
    Post::create_table(&db).await;
    Job::create_table(&db).await;

    let views = Views::new();

    let state = AppState::new(db, views);

    tokio::join!(Poll::spawn(state.clone()), Web::spawn(state.clone()));
}
