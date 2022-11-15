pub mod parse;
pub mod commands;
pub mod bot;

pub use commands::*;
pub use parse::parser;
pub use bot::{start_bot, channel_data};

pub async fn initialize() {
    start_bot().await;
}