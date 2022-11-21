pub struct ChannelInfo {
    category: Option<CategoryInfo>,
    channel: String,
    roles: Option<Vec<String>>,
    private: Option<bool>,
}

impl ChannelInfo {
    pub fn default() -> Self {
        ChannelInfo {
            category: None,
            channel: String::new(),
            roles: None,
            private: None,
        }
    }

    pub fn update(
        &self,
        category: CategoryInfo,
        channel: String,
        roles: Option<Vec<String>>,
    ) -> Self {
        ChannelInfo {
            category: Some(category),
            channel: channel,
            roles: roles,
            private: None,
        }
    }

    pub fn update_private(&mut self) {
        self.private = Some(true)
    }
}

#[derive(Debug, Clone)]
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

    pub fn update_name(&mut self, name: &str) {
        self.category = name.to_string()
    }

    pub fn update_roles(&mut self, roles: Vec<String>) {
        self.roles = roles
    }

    pub fn update_private(&mut self) {
        self.private = true
    }
}
