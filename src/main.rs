mod bot;
pub mod parser;
use bot::start_bot;

#[tokio::main]
async fn main() {
    start_bot().await;
}
