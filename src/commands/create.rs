use crate::accept;
use crate::bot::ChannelInfo;
use crate::parse::{parse_to_channel, parse_to_text};
use crate::utility::{get_guild_name, get_locked_parsedata};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
};
use serenity::model::user::User;
use serenity::prelude::*;
use serenity::Error;
use tracing::{error, info};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("create")
        .description("Create channels with given command")
        // remove command from working in dm
        .dm_permission(false)
        // add the option to accept a string
        .create_option(|option| {
            option
                .name("string")
                .description("command list for channel creation")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

pub fn run(options: &[CommandDataOption]) -> (Result<Vec<ChannelInfo>, &str>, String) {
    // get the actual string that was passed with the command
    let resolved = options
        .get(0)
        .expect("Some value")
        .resolved
        .as_ref()
        .expect("Some value");

    if let CommandDataOptionValue::String(value) = resolved {
        info!("'create' parsing data detected: {value}");

        // Parse the command string into more readable version which is the reply to the command
        // then parse again into struct for the program with work with
        let reply_string = parse_to_text(value.to_string());
        (parse_to_channel(value.to_string()), reply_string)
    } else {
        error!("Failed to get any parsing value. {resolved:?}");
        (
            Err("Failed to get values."),
            "No value was given. Parsing will happen here".to_string(),
        )
    }
}

pub async fn setup(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    user_data: User,
) -> Result<(), Error> {
    let interaction_message = command.get_interaction_response(&ctx.http).await?;
    // create a interaction tracker to the message
    let interaction_reply = interaction_message.await_component_interaction(ctx).await;

    let mut data_not_found = false;

    let guild_id = command.guild_id.unwrap_or(0.into());

    {
        // if reject button is used before the interaction, user data gets deleted from the memory
        // prevent progressing if data was not found or deleted
        let channel_data_lock = get_locked_parsedata(ctx).await;
        let channel_data = channel_data_lock.read().await;
        if !channel_data.contains_key(&user_data.id.0) {
            info!(
                "{}#{} with id {} tried to use a button on '{}' command. No user data found",
                user_data.name, user_data.discriminator, user_data.id.0, command.data.name,
            );
            data_not_found = true;
        }
    }

    if data_not_found {
        command
            .edit_original_interaction_response(&ctx, |response| {
                response
                    .content("Command interaction cancelled due to insufficient data".to_string())
                    .components(|comp| comp)
            })
            .await?;
        return Ok(());
    }

    if let Some(reply) = interaction_reply {
        info!(
            "Used '{}' button on '{}' used by {}#{} with id {} on guild {} with id {}",
            reply.data.custom_id,
            command.data.name,
            user_data.name,
            user_data.discriminator,
            user_data.id.0,
            get_guild_name(ctx, guild_id).await,
            guild_id
        );
        match reply.data.custom_id.as_str() {
            "Accept" => {
                command
                    .edit_original_interaction_response(&ctx, |response| {
                        response
                            .content("Command accepted. Execution will start now.")
                            .components(|comp| comp)
                    })
                    .await?;

                // Get the channel data sent by the user
                let channel_data_lock = get_locked_parsedata(ctx).await;
                let channel_data = channel_data_lock.read().await;

                // channel creation happens here
                let accept_run = accept::run(
                    &channel_data[&user_data.id.0],
                    guild_id,
                    ctx,
                    user_data.id.0,
                )
                .await;

                // drop the read lock
                drop(channel_data);

                match accept_run {
                    Ok(_) => {
                        command
                            .edit_original_interaction_response(&ctx, |response| {
                                response
                                    .content("Command executed successfully")
                                    // empty component so the button disappears
                                    .components(|comp| comp)
                            })
                            .await?;
                    }
                    Err(err) => {
                        error!("Error while doing Accept command. Error: {err}");
                        command
                            .edit_original_interaction_response(&ctx, |response| {
                                response
                                    .content(format!(
                                        "There was an error during the interaction. Error: {err}"
                                    ))
                                    // empty component so the button disappears
                                    .components(|comp| comp)
                            })
                            .await?;
                    }
                }
            }
            "Reject" => {
                let parsed_data_lock = get_locked_parsedata(ctx).await;

                {
                    // remove the user data
                    let mut parsed_data = parsed_data_lock.write().await;
                    if parsed_data.contains_key(&user_data.id.0) {
                        parsed_data.remove(&user_data.id.0);
                    }
                }

                command.delete_original_interaction_response(&ctx).await?;
            }
            _ => {}
        }
    }
    Ok(())
}
