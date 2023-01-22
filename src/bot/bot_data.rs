use crate::bot::ChannelInfo;
use serenity::model::Permissions;
use serenity::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ParsedData;

// Save user id as key with channel data as value in the struct
impl TypeMapKey for ParsedData {
    type Value = Arc<RwLock<HashMap<u64, Vec<ChannelInfo>>>>;
}

pub struct PermissionData;

impl TypeMapKey for PermissionData {
    type Value = Arc<RwLock<HashMap<u64, Permissions>>>;
}
