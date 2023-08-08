use crate::bot::ChannelInfo;

use serenity::model::prelude::ChannelType;
use std::collections::HashMap;
use tracing::info;

pub fn text_from_channels(channel_data: Vec<ChannelInfo>) -> String {
    let mut text_data: HashMap<String, Vec<String>> = HashMap::new();

    for channel in channel_data {
        let mut channel_name = format!("-ch {}", channel.channel);

        match channel.channel_type {
            ChannelType::News => channel_name.push_str(" -t ann"),
            ChannelType::Voice => channel_name.push_str(" -t voice"),
            _ => {}
        }

        if channel.private {
            channel_name.push_str(" -p");
        }
        info!("Parsed channel name: {}", channel_name);
        match channel.category {
            Some(category) => text_data
                .entry(format!("-cat {}", category.category))
                .or_insert_with(Vec::new)
                .push(channel_name),

            None => text_data
                .entry("".to_string())
                .or_insert_with(Vec::new)
                .push(channel_name),
        }
    }

    let mut full_text = String::new();

    match text_data.get("") {
        Some(d) => {
            full_text.push_str(&format!("{}\n\n", d.join(" ")));
            text_data.remove("");
        }
        None => {}
    };

    for (key, value) in text_data {
        full_text.push_str(&format!("{} {}\n\n", key, value.join(" ")));
    }

    full_text
}
