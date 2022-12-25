use serenity::model::channel::ChannelType;
#[derive(Debug, Clone)]
pub struct ChannelInfo {
    pub category: Option<CategoryInfo>,
    pub channel: String,
    pub roles: Option<Vec<String>>,
    pub channel_type: ChannelType,
    pub private: Option<bool>,
}

impl ChannelInfo {
    pub fn default() -> Self {
        ChannelInfo {
            category: None,
            channel: String::new(),
            roles: None,
            channel_type: ChannelType::Text,
            private: None,
        }
    }

    pub fn update(
        &mut self,
        category: Option<CategoryInfo>,
        channel: String,
        roles: Option<Vec<String>>,
    ) {
        self.category = category;
        self.channel = channel;
        self.roles = roles;
    }

    pub fn update_private(&mut self) {
        self.private = Some(true)
    }

    pub fn update_name_category(&mut self, name: String, category: Option<CategoryInfo>) {
        self.channel = name;
        self.category = category;
    }

    pub fn update_channel_type(&mut self, ch_type: &str) {
        match ch_type {
            "text" => self.channel_type = ChannelType::Text,
            "voice" => self.channel_type = ChannelType::Voice,
            "ann" => self.channel_type = ChannelType::News,
            _ => {}
        }
    }

    pub fn get_category_name(&self) -> Option<&str> {
        if let Some(category) = &self.category {
            Some(&category.category)
        } else {
            None
        }
    }

    pub fn get_category_roles(&self) -> &Option<Vec<String>> {
        if let Some(category) = &self.category {
            &category.roles
        } else {
            &None
        }
    }

    pub fn get_category_private(&self) -> bool {
        if let Some(category) = &self.category {
            category.private
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub struct CategoryInfo {
    pub category: String,
    roles: Option<Vec<String>>,
    private: bool,
}

impl CategoryInfo {
    pub fn default() -> Self {
        CategoryInfo {
            category: "None".to_string(),
            roles: None,
            private: false,
        }
    }

    pub fn update_name(&mut self, name: &str) {
        self.category = name.to_string()
    }

    pub fn update_roles(&mut self, roles: Vec<String>) {
        self.roles = Some(roles)
    }

    pub fn update_private(&mut self) {
        self.private = true
    }
}
