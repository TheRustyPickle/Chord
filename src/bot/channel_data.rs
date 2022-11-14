#[derive(Debug)]
pub struct ChannelInfo {
    category: Option<CategoryInfo>,
    channel: String,
    roles: Option<Vec<String>>,
    private: Option<bool>,
}

impl ChannelInfo {
    pub fn new(category: CategoryInfo, channel: String) -> Self {
        ChannelInfo {
            category: Some(category),
            channel: channel,
            roles: None,
            private: None,
        }
    }
}

#[derive(Debug)]
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
            private: false,
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
