use crate::bot::{ChannelInfo, ParsedData, PermissionData};
use serenity::builder::CreateButton;
use serenity::model::application::component::ButtonStyle;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Error;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::error;

/// returns the locked state of ParsedData struct
pub async fn get_locked_parsedata(ctx: &Context) -> Arc<RwLock<HashMap<u64, Vec<ChannelInfo>>>> {
    let read_data = ctx.data.read().await;
    read_data.get::<ParsedData>().unwrap().clone()
}

/// returns the locked state of PermissionData struct
pub async fn get_locked_permissiondata(
    ctx: &Context,
) -> Arc<RwLock<HashMap<u64, HashMap<String, Permissions>>>> {
    let read_data = ctx.data.read().await;
    read_data.get::<PermissionData>().unwrap().clone()
}

/// creates a button based on the style and the string that is passed
pub fn normal_button(name: &str, style: ButtonStyle) -> CreateButton {
    let mut b = CreateButton::default();
    b.custom_id(name);
    b.label(name);
    b.style(style);
    b
}

/// Replaces all spaces with dash and makes sure two dash is never together
pub fn polish_channel(name: &str) -> String {
    let mut output = String::new();
    let mut last_char_was_hyphen = false;

    for c in name.trim().chars() {
        if c.is_whitespace() {
            if !last_char_was_hyphen {
                output.push('-');
                last_char_was_hyphen = true;
            }
        } else {
            output.push(c);
            last_char_was_hyphen = false;
        }
    }
    output
}

/// Handles error from interactions that require extra steps
pub async fn handle_error(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    result: Result<(), Error>,
) {
    if let Err(e) = result {
        error!("Error acquired on command '{}': {e:?}", command.data.name);
        command
            .edit_original_interaction_response(&ctx, |response| {
                response
                    .content(format!(
                        "There was an error during the interaction. Error: {e:?}"
                    ))
                    .components(|comp| comp)
            })
            .await
            .unwrap();
    }
}

/// Tries to get guild name from GuildId
pub async fn get_guild_name(ctx: &Context, guild_id: GuildId) -> Option<String> {
    if let Some(guild) = guild_id.to_guild_cached(&ctx.cache) {
        return Some(guild.name);
    }

    if let Ok(guild) = guild_id.to_partial_guild(&ctx.http).await {
        return Some(guild.name);
    }
    Some("Not Found".to_string())
}

/// Returns the list of permissions used for /setup
pub fn get_perm_list<'a>() -> HashMap<&'a str, Permissions> {
    HashMap::from([
        ("Send Message", Permissions::SEND_MESSAGES),
        ("Manage Channel", Permissions::MANAGE_CHANNELS),
        ("Manage Roles", Permissions::MANAGE_ROLES),
        ("Attach Files", Permissions::ATTACH_FILES),
        ("Mention @everyone @here", Permissions::MENTION_EVERYONE),
        ("Manage Message", Permissions::MANAGE_MESSAGES),
        ("Read Message History", Permissions::READ_MESSAGE_HISTORY),
        ("Use Application Commands", Permissions::USE_SLASH_COMMANDS),
    ])
}
