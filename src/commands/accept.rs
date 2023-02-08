use crate::bot::ChannelInfo;
use crate::utility::get_locked_permissiondata;
use serenity::model::channel::ChannelType;
use serenity::model::channel::{PermissionOverwrite, PermissionOverwriteType};
use serenity::model::id::{ChannelId, GuildId, RoleId};
use serenity::model::Permissions;
use serenity::prelude::*;
use serenity::Error;
use std::collections::HashMap;
use tracing::info;

pub async fn run(
    data: &Vec<ChannelInfo>,
    guild_id: GuildId,
    ctx: &Context,
    user_id: u64,
) -> Result<(), Error> {
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
                    info!(
                        "'{name}' Category name already exists. {}",
                        all_category[name]
                    );
                    Some(all_category[name])
                } else {
                    let new_category = GuildId(guild_id.0)
                        .create_channel(&ctx.http, |c| {
                            c.name(name).kind(ChannelType::Category);

                            if channel.get_category_private() {
                                c.permissions(do_private(everyone_role.unwrap()));
                            }
                            c
                        })
                        .await?
                        .id;
                    all_category.insert(name.to_string(), new_category);
                    info!(
                        "'{name}' Category does not exist. Creating new category. {new_category}"
                    );
                    Some(new_category)
                }
            }
            None => None,
        };

        if let Some(cat_id) = category_id {
            let mut role_ids = Vec::new();

            if let Some(cat_roles) = channel.get_category_roles() {
                for (role_id, role) in all_roles.iter() {
                    if cat_roles.contains(&role.name) {
                        role_ids.push(role_id);
                    }
                }
            }

            if channel.get_category_private() {
                override_permissions_private(cat_id, role_ids, &ctx, &user_id).await?
            } else {
                override_permissions_public(cat_id, role_ids, &ctx, &user_id).await?
            }
        }

        let created_channel = GuildId(guild_id.0)
            .create_channel(&ctx.http, |c| {
                c.name(&channel.channel);

                if let Some(cat_id) = category_id {
                    c.category(cat_id);
                }
                c.kind(channel.channel_type);

                c
            })
            .await?;

        let mut channel_roles = vec![];

        // if we have to override permissions from category or add roles for a channel, remove all permissions that has been added
        if channel.roles != None {
            created_channel
                .id
                .edit(&ctx.http, |c| {
                    c.name(&channel.channel)
                        .permissions(remove_all_permissions(everyone_role.unwrap()))
                })
                .await?;
        } else {
            // if channel roles is empty, collect the category roles for adding to the channel
            if let Some(cat_roles) = channel.get_category_roles() {
                for (role_id, role) in all_roles.iter() {
                    if cat_roles.contains(&role.name) {
                        channel_roles.push(role_id);
                        info!("{} role found for channel", role.name);
                    }
                }
            }
        }

        // if either the channel or the category is private, make the channel private
        // this also removes all roles if added to the channel
        if channel.private != None || channel.get_category_private() {
            created_channel
                .id
                .edit(&ctx.http, |c| {
                    c.name(&channel.channel)
                        .permissions(do_private(everyone_role.unwrap()))
                })
                .await?;
        }

        // collect all role ids from channel if existing
        if let Some(ch_roles) = &channel.roles {
            for (role_id, role) in all_roles.iter() {
                if ch_roles.contains(&role.name) {
                    channel_roles.push(role_id);
                    info!("{} role found for channel", role.name);
                }
            }
        }

        // send out permission based on whether the channel was selected private or public
        if channel.private != None || channel.get_category_private() {
            override_permissions_private(created_channel.id, channel_roles, ctx, &user_id).await?;
        } else {
            override_permissions_public(created_channel.id, channel_roles, ctx, &user_id).await?;
        }
    }
    Ok(())
}

async fn override_permissions_public(
    channel_id: ChannelId,
    roles: Vec<&RoleId>,
    ctx: &Context,
    user_id: &u64,
) -> Result<(), Error> {
    let mut allow =
        Permissions::SEND_MESSAGES | Permissions::VIEW_CHANNEL | Permissions::READ_MESSAGE_HISTORY;
    let mut deny = Permissions::MENTION_EVERYONE
        | Permissions::MANAGE_CHANNELS
        | Permissions::MANAGE_MESSAGES
        | Permissions::MANAGE_GUILD
        | Permissions::MANAGE_ROLES
        | Permissions::CREATE_PUBLIC_THREADS
        | Permissions::CREATE_PRIVATE_THREADS;

    let locked_permissions = get_locked_permissiondata(ctx).await;

    {
        let saved_permissions = locked_permissions.read().await;
        if saved_permissions.contains_key(user_id) {
            allow = saved_permissions[user_id]["public_allow"];
            deny = saved_permissions[user_id]["public_deny"];
        }
    }

    for role in roles {
        let overwrite = PermissionOverwrite {
            allow,
            deny,
            kind: PermissionOverwriteType::Role(role.to_owned()),
        };
        channel_id.create_permission(&ctx.http, &overwrite).await?;
    }

    Ok(())
}

async fn override_permissions_private(
    channel_id: ChannelId,
    roles: Vec<&RoleId>,
    ctx: &Context,
    user_id: &u64,
) -> Result<(), Error> {
    let mut allow =
        Permissions::SEND_MESSAGES | Permissions::VIEW_CHANNEL | Permissions::READ_MESSAGE_HISTORY;
    let mut deny = Permissions::empty();

    let locked_permissions = get_locked_permissiondata(ctx).await;

    {
        let saved_permissions = locked_permissions.read().await;
        if saved_permissions.contains_key(user_id) {
            allow = saved_permissions[user_id]["private_allow"];
            deny = saved_permissions[user_id]["private_deny"];
        }
    }

    for role in roles {
        let overwrite = PermissionOverwrite {
            allow,
            deny,
            kind: PermissionOverwriteType::Role(role.to_owned()),
        };
        channel_id.create_permission(&ctx.http, &overwrite).await?;
    }

    Ok(())
}

fn do_private(role: &RoleId) -> Vec<PermissionOverwrite> {
    let allow = Permissions::empty();
    let deny = Permissions::VIEW_CHANNEL;

    vec![PermissionOverwrite {
        allow,
        deny,
        kind: PermissionOverwriteType::Role(role.to_owned()),
    }]
}

fn remove_all_permissions(role: &RoleId) -> Vec<PermissionOverwrite> {
    let allow = Permissions::empty();
    let deny = Permissions::empty();

    vec![PermissionOverwrite {
        allow,
        deny,
        kind: PermissionOverwriteType::Role(role.to_owned()),
    }]
}
