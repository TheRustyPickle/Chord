use crate::bot::{ParsedData, PermissionData};
use crate::utility::{get_guild_name, get_locked_parsedata, handle_error, normal_button};
use crate::{check_setup, create, example, help, setup, start};
use serenity::async_trait;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::application::command::Command;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

struct Handler;

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
            let user_data = command.user.clone();

            info!(
                "Slash command '{}' used by {}#{} with id {} on guild {} with id {}",
                command.data.name,
                user_data.name,
                user_data.discriminator,
                user_data.id.0,
                get_guild_name(&ctx, command.guild_id.unwrap())
                    .await
                    .unwrap(),
                command.guild_id.unwrap()
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

pub async fn start_bot() {
    // initialize trace logging
    let mut env_filter = EnvFilter::from_default_env();
    if let Ok(level) = std::env::var("RUST_LOG") {
        if level == "debug" {
            env_filter = env_filter
                .add_directive(format!("{}=debug", env!("CARGO_PKG_NAME")).parse().unwrap())
                .add_directive(LevelFilter::ERROR.into())
                .add_directive(LevelFilter::INFO.into());
        }
    } else {
        env_filter = env_filter.add_directive(LevelFilter::INFO.into());
    }
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // get the bot token from environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new().configure(|c| c.owners(owners).prefix("!"));

    // allow only two intents to prevent flooding
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // initialize the struct data so if we fetch, it does not crash.
    {
        let mut data = client.data.write().await;
        data.insert::<ParsedData>(Arc::new(RwLock::new(HashMap::new())));
    }

    {
        let mut data = client.data.write().await;
        data.insert::<PermissionData>(Arc::new(RwLock::new(HashMap::new())));
    }

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
