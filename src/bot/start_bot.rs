use crate::utility::{get_guild_name, get_locked_parsedata, handle_error, normal_button};
use crate::{check_setup, create, example, help, setup, start};
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::{debug, error, info};

// TODO add sqlite db support for saving data

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Create the designated slash commands on bot start
    async fn ready(&self, context: Context, _ready: Ready) {
        let status = Command::set_global_application_commands(&context.http, |commands| {
            commands
                .create_application_command(|command| create::register(command))
                .create_application_command(|command| help::register(command))
                .create_application_command(|command| start::register(command))
                .create_application_command(|command| example::register(command))
                .create_application_command(|command| setup::register(command))
                .create_application_command(|command| check_setup::register(command))
        })
        .await;

        // exit the app if command creation is failed
        if let Err(e) = status {
            error!("Client crashed. Exiting. Reason: {e}");
            std::process::exit(1)
        }
        info!("The bot is online");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // contains various user data
            let user_data = command.user.clone();

            let guild_id = command.guild_id.unwrap_or(0.into());
            info!(
                "Slash command '{}' used by {} with id {} on guild {} with id {}",
                command.data.name,
                user_data.name,
                user_data.id.0,
                get_guild_name(&ctx, guild_id).await,
                guild_id
            );

            let mut parse_success = false;

            let content = match command.data.name.as_str() {
                "create" => {
                    // run the command, get channel data as result and the string to send as a reply
                    let (parsing_status, command_reply) = create::run(&command.data.options);

                    // if channel data properly acquired, unlock struct value, write the channel data in hashmap and close
                    if let Ok(parsed) = parsing_status {
                        debug!("Inserting parsed data: {parsed:#?}");
                        parse_success = true;
                        let parsed_data_lock = get_locked_parsedata(&ctx).await;

                        {
                            let mut parsed_data = parsed_data_lock.write().await;
                            parsed_data.insert(user_data.id.0, parsed);
                        }
                    }

                    command_reply
                }
                // returns a string which is sent as the reply
                "help" => help::run(&command.data.options),
                "start" => start::run(&command.data.options),
                "example" => example::run(&command.data.options),
                "setup" => setup::run(&command.data.options),
                "check_setup" => {
                    check_setup::run(&command.data.options, &ctx, user_data.id.0).await
                }
                _ => "Command not found".to_string(),
            };

            // add Accept and Reject button if parsing was successful and if 'create' was called
            // ephemeral makes the message visible only to the command executor
            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            if command.data.name == "create" && parse_success {
                                message.content(content).ephemeral(true).components(|c| {
                                    c.create_action_row(|row| {
                                        row.add_button(normal_button(
                                            "Accept",
                                            ButtonStyle::Primary,
                                        ));
                                        row.add_button(normal_button("Reject", ButtonStyle::Danger))
                                    })
                                })
                            } else {
                                message.content(content).ephemeral(true)
                            }
                        })
                })
                .await
                .unwrap();

            // further interaction from the command is handled from here
            match command.data.name.as_str() {
                "create" => {
                    if parse_success {
                        handle_error(
                            &ctx,
                            &command,
                            create::setup(&ctx, &command, user_data).await,
                        )
                        .await;
                    }
                }
                "setup" => {
                    handle_error(
                        &ctx,
                        &command,
                        setup::setup(&ctx, &command, user_data).await,
                    )
                    .await
                }
                _ => {}
            }
        }
    }
}
