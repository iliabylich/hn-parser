mod config;
use config::Config;

mod poll;
use poll::Poll;

mod web;
use web::Web;

mod database;
use database::Database;

mod schema;
use schema::Schema;

mod state;
use state::AppState;

mod views;
use views::Views;

mod job;
mod post;

mod hn_client;

mod fixture;

#[tokio::main]
async fn main() {
    Config::load();
    println!("Running with config {:?}", Config::global());

    let db = Database::new().await;
    Schema::apply(&db).await;

    let views = Views::new();

    let state = AppState::new(db, views);

    tokio::join!(Poll::spawn(state.clone()), Web::spawn(state.clone()));
}
