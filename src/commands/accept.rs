use crate::bot::{CategoryInfo, ChannelInfo};
use serenity::model::channel::ChannelType;
use serenity::model::channel::{PermissionOverwrite, PermissionOverwriteType};
use serenity::model::id::{ChannelId, GuildId, RoleId};
use serenity::model::{ModelError, Permissions};
use serenity::{prelude::*};
use std::collections::HashMap;
use std::error::Error;
use tracing::info;

pub async fn run(
    data: &Vec<ChannelInfo>,
    guild_id: GuildId,
    ctx: &Context,
) -> Result<(), Box<dyn Error>> {
    let mut all_category = HashMap::new();
    let all_roles = guild_id.roles(&ctx.http).await?;

    let mut everyone_role = None;
    
    for (role_id, role) in &all_roles {
        if role.name == "@everyone" {
            everyone_role = Some(role_id)
        }
    }

    let all_guild_channels = ctx.http.get_channels(guild_id.0).await?;

    for guild in all_guild_channels {
        if guild.kind == ChannelType::Category {
            all_category.insert(guild.name, guild.id);
        }
    }

    for channel in data {
        let category_id = match channel.get_category_name() {
            Some(name) => {
                if all_category.contains_key(name) {
                    Some(all_category[name])
                } else {
                    Some(
                        GuildId(guild_id.0)
                            .create_channel(&ctx.http, |c| {
                                c.name(name).kind(ChannelType::Category);
                                
                                if channel.get_category_private() {
                                    c.permissions(do_private_category(everyone_role.unwrap()));
                                }
                                c
                            })
                            .await?
                            .id,
                    )
                }
            }
            None => None,
        };

        if let Some(cat_id) = category_id {
            if let Some(cat_roles) = channel.get_category_roles() {

                let mut role_ids = Vec::new();

                for (role_id, role) in all_roles.iter() {

                    if cat_roles.contains(&role.name) {
                        role_ids.push(role_id);
                    }
                }
                role_ids.push(everyone_role.unwrap());

                override_permissions_public(cat_id, role_ids, &ctx).await?
            }
        }

        let create_channel = GuildId(guild_id.0)
            .create_channel(&ctx.http, |c| {
                c.name(&channel.channel);

                if let Some(cat_id) = category_id {
                    c.category(cat_id);
                }

                c
            })
            .await.unwrap();
    }
    Ok(())
}

async fn override_permissions_public(channel_id: ChannelId, roles: Vec<&RoleId>, ctx: &Context) -> Result<(), Box<dyn Error>> {
    let allow =
        Permissions::SEND_MESSAGES | Permissions::VIEW_CHANNEL | Permissions::READ_MESSAGE_HISTORY;
    let deny = Permissions::MENTION_EVERYONE
        | Permissions::MANAGE_CHANNELS
        | Permissions::MANAGE_MESSAGES
        | Permissions::MANAGE_GUILD
        | Permissions::MANAGE_ROLES
        | Permissions::CREATE_PUBLIC_THREADS
        | Permissions::CREATE_PRIVATE_THREADS;

    for role in roles {
        let overwrite = PermissionOverwrite {
            allow,
            deny,
            kind: PermissionOverwriteType::Role(role.to_owned())
        };
        channel_id.create_permission(&ctx.http, &overwrite).await?;
    }

    Ok(())
}

async fn override_permissions_private(channel_id: ChannelId, roles: Vec<&RoleId>, ctx: &Context) -> Result<(), Box<dyn Error>> {
    let allow =
        Permissions::SEND_MESSAGES | Permissions::VIEW_CHANNEL | Permissions::READ_MESSAGE_HISTORY;
    let deny = Permissions::empty();

    for role in roles {
        let overwrite = PermissionOverwrite {
            allow,
            deny,
            kind: PermissionOverwriteType::Role(role.to_owned())
        };
        channel_id.create_permission(&ctx.http, &overwrite).await?;
    }

    Ok(())
}

fn do_private_category(role: &RoleId) -> Vec<PermissionOverwrite> {
    let allow = Permissions::empty();
    let deny = Permissions::VIEW_CHANNEL;

    vec![PermissionOverwrite {
        allow,
        deny,
        kind: PermissionOverwriteType::Role(role.to_owned())
    }]
}