mod config;
use config::Config;

#[tokio::main]
async fn main() {
    Config::load();
    println!("Running with config {:?}", Config::global());
}
