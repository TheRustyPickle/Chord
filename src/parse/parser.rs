use std::collections::HashMap;

pub fn parse_input<'a>(
    mut input: String,
) -> Result<HashMap<&'a str, Vec<String>>, HashMap<&'a str, Vec<String>>> {
    let mut collected_data = HashMap::new();

    let sensitive_string = ["-ch", "-cat", "-r", "-p"];
    let mut parsed_successfully = false;

    for _num in 0..collected_data.len() + 10 {
        if input.is_empty() {
            parsed_successfully = true;
            break;
        }

        let splitted_data: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();
        let data = &splitted_data[0];
        match data.trim() {
            "-cat" => {
                input = input.replace(data, "").trim().to_string();
                let mut category_name = String::new();
                for i in 1..splitted_data.len() {
                    if !splitted_data[i].starts_with('-') {
                        category_name.push_str(&splitted_data[i]);
                        category_name.push(' ');
                    } else {
                        break;
                    }
                }
                category_name = category_name.trim().to_string();
                input = input.replace(&category_name, "").trim().to_string();
                collected_data.insert("category", vec![category_name]);
            }

            "-p" => {
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

                let comma_splitted: Vec<String> =
                    role_input.split(", ").map(|s| s.to_string()).collect();

                let mut all_roles = Vec::new();

                for role in comma_splitted {
                    if !role.starts_with('-') {
                        all_roles.push(role);
                    } else {
                        break;
                    }
                }
                input = input.replace(&role_input, "").trim().to_string();
                collected_data.insert("roles", all_roles);
            }
            "-ch" => {
                input = input.replacen(data, "", 1).trim().to_string();
                let mut channels = Vec::new();
                let mut separated: Vec<String> =
                    input.split(" | ").map(|s| s.to_string()).collect();

                for i in 0..separated.len() {
                    let split: Vec<&str> = separated[i].split(' ').collect();
                    if sensitive_string.contains(&split[0]) {
                        separated.remove(i);
                        break;
                    }
                }

                // TODO do not allow spaces between channel name. use replacen 1

                let mut channel_input = String::new();
                for i in 0..separated.len() {
                    channel_input.push_str(&separated[i]);
                    if i != separated.len() - 1 {
                        channel_input.push_str(" | ");
                    }

                    channels.push(separated[i].to_owned())
                }

                input = input.replace(&channel_input, "").trim().to_string();

                collected_data.insert("channels", channels);
            }
            _ => {}
        }
    }
    if !parsed_successfully {
        return Err(collected_data);
    }
    Ok(collected_data)
}
