use crate::bot::{CategoryInfo, ChannelInfo};
use crate::parse::parse_input;
use std::collections::HashMap;

const SENSITIVE_STRING: [&str; 4] = ["-ch", "-cat", "-r", "-p"];

pub fn parse_to_channel<'a>(mut unparsed: String) -> Result<Vec<ChannelInfo>, &'a str> {
    unparsed = unparsed.trim().replace('\n', " ");

    let mut all_channels: Vec<ChannelInfo> = Vec::new();

    let splitted_data: Vec<String> = unparsed.split("-cat ").map(|s| s.to_string()).collect();

    for split in splitted_data {
        if split.is_empty() {
            continue;
        }
        let parsed_data = parse_input(format!("-cat {split}"));
        match parsed_data {
            Ok(data) => {
                let channel_data = get_base_data(data);
                if let Ok(data) = channel_data {
                    for i in data {
                        all_channels.push(i)
                    }
                } else {
                    return Err("Could not properly parse input");
                }
            }
            Err(_) => return Err("Could not properly parse input"),
        }
    }
    Ok(all_channels)
}

fn get_base_data(data: HashMap<&str, Vec<String>>) -> Result<Vec<ChannelInfo>, &str> {
    let mut category = None;
    let mut all_channels: Vec<ChannelInfo> = Vec::new();

    if data.contains_key("category") && !data["category"][0].is_empty() {
        let mut cu_category = CategoryInfo::default();
        cu_category.update_name(&data["category"][0]);

        if data.contains_key("roles") {
            cu_category.update_roles(data["roles"].to_owned());
        }

        if data.contains_key("private") {
            cu_category.update_private();
        }
        category = Some(cu_category)
    }

    if data.contains_key("channels") {
        for channel in &data["channels"] {
            let mut channel_data = ChannelInfo::default();

            let mut channel_name_unparsed = String::new();

            for word in channel.split(' ').collect::<Vec<&str>>() {
                if SENSITIVE_STRING.contains(&word) {
                    break;
                }
                if word.starts_with("|") {
                    break;
                }

                channel_name_unparsed.push_str(&format!(" {word}"));
            }
            channel_name_unparsed = channel_name_unparsed.trim().to_string();

            let channel_name = channel_name_unparsed.replace(" ", "-");

            let channel = channel
                .replace(&channel_name_unparsed, "")
                .trim()
                .to_string();

            if !channel.is_empty() {
                let parsed_channel = parse_input(channel.to_string());
                match parsed_channel {
                    Ok(data) => {
                        channel_data.update(
                            category.clone(),
                            channel_name,
                            data.get("roles").cloned(),
                        );
                        if data.contains_key("private") {
                            channel_data.update_private()
                        }
                        all_channels.push(channel_data);
                    }
                    Err(_) => return Err("Could not parse channel"),
                }
            } else {
                channel_data.update_name_category(channel_name, category.clone());
                all_channels.push(channel_data);
            }
        }
    }
    Ok(all_channels)
}
