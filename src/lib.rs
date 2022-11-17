pub mod bot;
pub mod commands;
pub mod parse;

pub use bot::{
    channel_data::{CategoryInfo, ChannelInfo},
    start_bot,
};
pub use commands::*;
pub use parse::parser;

pub async fn initialize() {
    start_bot().await;
}
