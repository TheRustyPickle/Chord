use crate::bot::{ChannelInfo, ParsedData, PermissionData};
use serenity::builder::CreateButton;
use serenity::model::application::component::ButtonStyle;
use serenity::model::Permissions;
use serenity::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn get_locked_parsedata(ctx: &Context) -> Arc<RwLock<HashMap<u64, Vec<ChannelInfo>>>> {
    let read_data = ctx.data.read().await;
    read_data.get::<ParsedData>().unwrap().clone()
}

pub async fn get_locked_permissiondata(
    ctx: &Context,
) -> Arc<RwLock<HashMap<u64, HashMap<String, Permissions>>>> {
    let read_data = ctx.data.read().await;
    read_data.get::<PermissionData>().unwrap().clone()
}

// creates a button based on the style and the text that is passed
pub fn normal_button(name: &str, style: ButtonStyle) -> CreateButton {
    let mut b = CreateButton::default();
    b.custom_id(name);
    b.label(name);
    b.style(style);
    b
}
