mod config;
use config::Config;

mod poll;
use poll::Poll;

mod ui;
use ui::UI;

#[tokio::main]
async fn main() {
    Config::load();
    println!("Running with config {:?}", Config::global());

    tokio::join!(Poll::spawn(), UI::spawn());
}
