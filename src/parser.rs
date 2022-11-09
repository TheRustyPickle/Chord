#![warn(dead_code)]

#[derive(Debug)]
pub struct ChannelInfo {
    category: CategoryInfo,
    channel: String,
    roles: Option<Vec<String>>,
    private: Option<bool>,
    
}

impl ChannelInfo {
    pub fn new(category: CategoryInfo, channel: String) -> Self {
        ChannelInfo { 
            category: category, 
            channel: channel, 
            roles: None, 
            private: None}
    }
}

#[derive(Clone, Debug)]
pub struct CategoryInfo {
    category: String,
    roles: Vec<String>,
    private: bool,
}

impl CategoryInfo {
    pub fn default() -> Self {
        CategoryInfo {
            category: "None".to_string(),
            roles: Vec::new(),
            private: false
        }
    }

    pub fn update_name(&mut self, name: String) {
        self.category = name.to_string()
    }

    pub fn update_roles(&mut self, roles: Vec<String>) {
        self.roles = roles
    }

    pub fn update_private(&mut self, private: bool) {
        self.private = private
    }
}

// TODO make the function recursive so it can be used for channel -commands instead of only category

pub fn parse_input(mut input: String) -> Vec<ChannelInfo> {
    let mut working_category = CategoryInfo::default();
    let mut channel_data: Vec<ChannelInfo> = Vec::new();
    let sensitive_string = ["-ch", "-cat", "-r", "-p"];

    while !input.is_empty() {
        let splitted_data: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();
        let data = &splitted_data[0];
        match data.trim() {
            "-cat" => {

                input = input.replace(&format!("{data}"), "").trim().to_string();
                let mut category_name = String::new();
                for i in 1..splitted_data.len() {
                    if !splitted_data[i].starts_with("-") {
                        category_name.push_str(&splitted_data[i]);
                        category_name.push_str(" ");
                    } else {
                        break
                    }
                }
                category_name = category_name.trim().to_string();
                input = input.replace(&format!("{category_name} "), "");
                working_category.update_name(category_name);
            }

            "-p" => {
                input = input.replace(&format!("{data} "), "");
                working_category.update_private(true);
            }
            "-r" => {
                input = input.replace(&format!("{data} "), "");

                let mut role_input = String::new();

                for i in 1..splitted_data.len() {
                    if !splitted_data[i].starts_with("-") {
                        role_input.push_str(&splitted_data[i]);
                        role_input.push_str(" ");
                    } else {
                        break
                    }
                }
                role_input = role_input.trim().to_string();

                let comma_splitted: Vec<String> = role_input.split(", ").map(|s| s.to_string()).collect();

                let mut all_roles = Vec::new();

                for role in comma_splitted {
                    if !role.starts_with("-") {
                        all_roles.push(role);
                        
                    } else {
                        break
                    }
                }
                input = input.replace(&role_input, "").trim().to_string();
                working_category.update_roles(all_roles)
            }
            "-ch" => {
                input = input.replace(&format!("{data} "), "");
                let mut channels = Vec::new();
                for i in 1..splitted_data.len() {
                    if !sensitive_string.contains(&splitted_data[i].as_str()) {
                        channels.push(&splitted_data[i])
                    } else {
                        break
                    }
                }

                for channel in channels {
                    input = input.replace(&format!("{channel}"), "").trim().to_string();
                    channel_data.push(ChannelInfo::new(working_category.to_owned(), channel.to_string()))
                }

            }
            _ => {}
        }
    }
    for i in &channel_data {
        println!("{:?}", i);
    }
    channel_data
}
