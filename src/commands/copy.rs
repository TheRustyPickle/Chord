use crate::parse::text_from_channels;
use crate::utility::get_id_to_channel_name;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption,
};
use serenity::model::prelude::ChannelType;
use serenity::prelude::*;
use serenity::Error;
use tracing::{error, info};

use crate::bot::{CategoryInfo, ChannelInfo};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("copy")
        .dm_permission(false)
        .description("Copy's a guild's channel structure")
}

pub async fn run(_options: &[CommandDataOption]) -> String {
    "Processing channels...".to_string()
}

pub async fn setup(command: &ApplicationCommandInteraction, ctx: &Context) -> Result<(), Error> {
    let guild_id = command.guild_id.unwrap();

    let mut gathered_channels = Vec::new();

    match guild_id.channels(&ctx).await {
        Ok(channel_data) => {
            for (_, channel) in channel_data {
                match channel.kind {
                    ChannelType::Text
                    | ChannelType::Private
                    | ChannelType::Voice
                    | ChannelType::News => {
                        info!(
                            "Found channel {} with id {} with category id {:?}",
                            channel.name, channel.id, channel.parent_id
                        );
                        let mut channel_info = ChannelInfo::new();
                        channel_info.update(None, channel.name, None);
                        channel_info.update_channel_type_with_type(channel.kind);

                        match channel.parent_id {
                            Some(id) => match get_id_to_channel_name(id, &ctx).await {
                                Some(name) => {
                                    info!("Category name found: {name}");
                                    let mut category = CategoryInfo::new();
                                    category.update_name(&name);
                                    channel_info.update_category(Some(category));
                                }
                                None => error!("Failed to get category name"),
                            },
                            None => {}
                        }
                        gathered_channels.push(channel_info);
                    }
                    _ => {}
                }
            }
        }
        Err(err) => {
            error!("Failed to get guild channel data. Reason: {:?}", err);
            command
                .edit_original_interaction_response(&ctx, |resp| {
                    resp.content(format!(
                        "Failed to get guild channel data. Reason: {:?}",
                        err
                    ))
                })
                .await?;
        }
    }

    if gathered_channels.is_empty() {
        info!("Could not find any guild channels on {}", guild_id);
        command
            .edit_original_interaction_response(&ctx, |resp| {
                resp.content("Failed to get guild channel data. Reason: could not see any channels")
            })
            .await?;
    } else {
        let parsed_text = text_from_channels(gathered_channels);
        command.edit_original_interaction_response(&ctx, |resp| {
            resp.content(format!("Use the following text with /create command to copy the channels.\n\n⚠️ Category location may differ. Move them as necessary before executing ⚠️\n\n```\n{parsed_text}\n```"))
        }).await?;
    }
    Ok(())
}
