use crate::accept;
use crate::bot::ChannelInfo;
use crate::parse::{parse_to_channel, parse_to_text};
use crate::utility::get_locked_parsedata;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
};
use serenity::model::user::User;
use serenity::prelude::*;

use tracing::{error, info, instrument};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("create")
        .description("Command for creating channels")
        .dm_permission(false)
        .create_option(|option| {
            option
                .name("string")
                .description("command list for channel creation")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

#[instrument]
pub fn run(options: &[CommandDataOption]) -> (Result<Vec<ChannelInfo>, &str>, String) {
    let resolved = options
        .get(0)
        .expect("Some value")
        .resolved
        .as_ref()
        .expect("Some value");
    if let CommandDataOptionValue::String(value) = resolved {
        info!("'Create' parsing data detected: {value}");

        let reply_string = parse_to_text(value.to_string());
        (parse_to_channel(value.to_string()), reply_string)
    } else {
        error!("Failed to get any parsing value. {resolved:?}");
        (
            Err("Failed to get values. "),
            "No value was given. Parsing will happen here".to_string(),
        )
    }
}

pub async fn setup(ctx: &Context, command: ApplicationCommandInteraction, user_data: User) {
    let interaction_message = command.get_interaction_response(&ctx.http).await.unwrap();
    // create a interaction tracker to the message
    let interaction_reply = interaction_message.await_component_interaction(&ctx).await;

    match interaction_reply {
        // start matching button id
        Some(reply) => {
            match reply.data.custom_id.as_str() {
                "Accept" => {
                    info!(
                        "Used 'Accept' button on '{}' used by {}#{} with id {} on {:?}",
                        command.data.name,
                        user_data.name,
                        user_data.discriminator,
                        user_data.id.0,
                        command.guild_id
                    );
                    command
                        .edit_original_interaction_response(&ctx, |response| {
                            response
                                .content("Command accepted. Execution will start now.")
                                .components(|comp| comp)
                        })
                        .await
                        .unwrap();

                    // read the data that was saved inside the hashmap to get the channel data
                    let channel_data_lock = get_locked_parsedata(&ctx).await;
                    let channel_data = { channel_data_lock.read().await };

                    match accept::run(
                        &channel_data[&user_data.id.0],
                        command.guild_id.unwrap(),
                        &ctx,
                    )
                    .await
                    {
                        Ok(_) => {
                            command
                                .edit_original_interaction_response(&ctx, |response| {
                                    response
                                        .content(format!("Command executed successfully"))
                                        .components(|comp| comp)
                                })
                                .await
                                .unwrap();
                        }
                        Err(err) => {
                            info!("Error while doing Accept command. Error: {err}");
                            command
                    .edit_original_interaction_response(&ctx, |response| {
                        response.content(format!("There was an error during the interaction. Error: {err}"))
                        .components(|comp| {comp})
                    })
                    .await
                    .unwrap();
                        }
                    }
                }
                "Reject" => {
                    info!(
                        "Used 'Reject' button on '{}' used by {}#{} with id {} on {:?}",
                        command.data.name,
                        user_data.name,
                        user_data.discriminator,
                        user_data.id.0,
                        command.guild_id
                    );
                    // TODO delete data from hashmap
                    command
                        .delete_original_interaction_response(&ctx)
                        .await
                        .unwrap();
                }
                _ => {}
            }
        }
        None => {}
    }
}
