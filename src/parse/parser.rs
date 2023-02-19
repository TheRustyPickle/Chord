use std::collections::HashMap;
use tracing::{debug, error, info};
pub fn parse_input<'a>(
    mut input: String,
) -> Result<HashMap<&'a str, Vec<String>>, HashMap<&'a str, Vec<String>>> {
    let mut collected_data = HashMap::new();

    let sensitive_string = ["-ch", "-cat", "-r", "-p", "-t"];
    let mut parsed_successfully = false;

    // The loop goes through each part of the string and once a part is parsed
    // that part is removed from the string. So ideally, at the end the whole string should become empty.
    // Using for loops here otherwise it will get struck if some part is not parsed.
    for _num in 0..input.len() {
        if input.is_empty() {
            parsed_successfully = true;
            break;
        }

        let splitted_data: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();
        let data = &splitted_data[0];
        debug!("Splitted data status: {splitted_data:?}\nCurrently checking: {data}");
        match data.trim() {
            "-cat" => {
                input = input.replacen(data, "", 1).trim().to_string();
                let mut category_name = String::new();
                for i in 1..splitted_data.len() {
                    if !sensitive_string.contains(&splitted_data[i].as_str()) {
                        category_name.push_str(&splitted_data[i]);
                        category_name.push(' ');
                    } else {
                        break;
                    }
                }
                category_name = category_name.trim().to_string();
                info!("Category parsed: {category_name}");
                input = input.replacen(&category_name, "", 1).trim().to_string();
                collected_data.insert("category", vec![category_name]);
            }

            "-p" => {
                info!("Private flag parsed");
                input = input.replacen(data, "", 1).trim().to_string();
                collected_data.insert("private", vec!["true".to_string()]);
            }
            "-r" => {
                input = input.replacen(data, "", 1).trim().to_string();

                let mut role_input = String::new();

                for i in 1..splitted_data.len() {
                    if !sensitive_string.contains(&splitted_data[i].as_str()) {
                        role_input.push_str(&splitted_data[i]);
                        role_input.push(' ');
                    } else {
                        break;
                    }
                }
                role_input = role_input.trim().to_string();

                let comma_splitted: Vec<&str> = role_input.split(",").collect();

                let mut all_roles = Vec::new();

                for role in comma_splitted {
                    all_roles.push(role.trim().to_string());
                }
                input = input.replacen(&role_input, "", 1).trim().to_string();
                info!("Roles parsed: {all_roles:?}");
                collected_data.insert("roles", all_roles);
            }
            "-ch" => {
                input = input.replacen(data, "", 1).trim().to_string();
                let mut ch_found = 0;

                let separated_string: Vec<&str> = input.split(" ").collect();
                let mut collected_channel_data = Vec::new();
                let mut current_channel = String::new();
                for sep in separated_string {
                    match sep {
                        "-cat" => break,
                        "-ch" => {
                            ch_found += 1;
                            collected_channel_data.push(current_channel.trim().to_string());
                            current_channel = String::new()
                        }
                        _ => current_channel.push_str(&format!("{sep} ")),
                    }
                }
                if !current_channel.is_empty() {
                    collected_channel_data.push(current_channel.trim().to_string());
                }
                for i in &collected_channel_data {
                    input = input.replacen(i, "", 1).trim().to_string()
                }
                info!("Channel parsed: {collected_channel_data:?}");
                collected_data.insert("channels", collected_channel_data);
                input = input.replacen("-ch", "", ch_found).trim().to_string();
            }
            "-t" => {
                input = input.replacen(data, "", 1).trim().to_string();
                let channel_type = &splitted_data[1].trim();
                collected_data.insert(
                    "channel_type",
                    vec![channel_type.to_lowercase().to_string()],
                );
                info!("Channel Type parsed: {channel_type}");
                input = input.replacen(channel_type, "", 1).trim().to_string();
            }
            _ => {}
        }
    }
    if !parsed_successfully {
        error!("Failed parsing data successfully. Collected data: {collected_data:#?}");
        return Err(collected_data);
    }
    debug!("Data parsed: {collected_data:#?}");
    Ok(collected_data)
}
