#[derive(Debug, Clone)]
pub struct ChannelInfo {
    pub category: Option<CategoryInfo>,
    pub channel: String,
    pub roles: Option<Vec<String>>,
    pub private: Option<bool>,
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
        category: Option<CategoryInfo>,
        channel: String,
        roles: Option<Vec<String>>,
    ) -> Self {
        ChannelInfo {
            category,
            channel,
            roles,
            private: None,
        }
    }

    pub fn update_private(&mut self) {
        self.private = Some(true)
    }

    pub fn update_name_category(&mut self, name: String, category: Option<CategoryInfo>) {
        self.channel = name;
        self.category = category;
    }

    pub fn get_category_name(&self) -> Option<&str> {
        if let Some(category) = &self.category {
            Some(&category.category)
        } else { None }
    }
}

#[derive(Debug, Clone)]
pub struct CategoryInfo {
    pub category: String,
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
