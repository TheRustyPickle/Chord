use crate::bot::ChannelInfo;
use serenity::model::Permissions;
use serenity::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ParsedData;

// Saves user id as key with channel data as value that will be used to create the channels 
impl TypeMapKey for ParsedData {
    type Value = Arc<RwLock<HashMap<u64, Vec<ChannelInfo>>>>;
}

pub struct PermissionData;

// Saves user id as key for the first hashmap, second hashmap stores permissions set by the user for public, private channels
// Only added keys for the second hashmap are: 
// public_allow : the permissions allowed for public channels
// public_deny : the permissions denied for public channels
// private_allow : the permissions allowed for private channels
// private_deny : the permissions denied for private channels
impl TypeMapKey for PermissionData {
    type Value = Arc<RwLock<HashMap<u64, HashMap<String, Permissions>>>>;
}
