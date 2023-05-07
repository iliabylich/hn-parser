mod config;
use config::Config;

mod poll;
use poll::Poll;

mod ui;
use ui::UI;

mod database;
use database::Database;

mod schema;
use schema::Schema;

mod state;
use state::AppState;

mod job;
mod post;

mod hn_client;

#[tokio::main]
async fn main() {
    Config::load();
    println!("Running with config {:?}", Config::global());

    let db = Database::new().await;
    Schema::apply(&db).await;

    let state = AppState::new(db);

    tokio::join!(Poll::spawn(state.clone()), UI::spawn(state.clone()));
}
