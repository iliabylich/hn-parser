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

mod job;
mod post;

mod hn_client;

#[tokio::main]
async fn main() {
    Config::load();
    println!("Running with config {:?}", Config::global());

    let db = Database::new().await;
    Schema::apply(&db).await;

    let post = hn_client::HnClient::get_latest_post().await;
    println!("Latest post: {:?}", post);

    tokio::join!(Poll::spawn(), UI::spawn());
}
