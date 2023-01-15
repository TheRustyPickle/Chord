use crate::bot::ChannelInfo;
use crate::{create, help, start, example};
use serenity::async_trait;
use serenity::builder::CreateButton;
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
use tracing::{error, info, instrument};

pub struct ParsedData;

// Save user id as key with channel data as value in the struct
impl TypeMapKey for ParsedData {
    type Value = Arc<RwLock<HashMap<u64, Vec<ChannelInfo>>>>;
}

// creates a button based on the style and the text that is passed
fn normal_button(name: &str, style: ButtonStyle) -> CreateButton {
    let mut b = CreateButton::default();
    b.custom_id(name);
    b.label(name);
    b.style(style);
    b
}

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
        })
        .await;

        // exit the app if command creation is failed
        if let Err(e) = status {
            error!("Client crashed. Exiting. Reason: {e}");
            std::process::exit(1)
        }
        info!("The bot is online");
    }

    //#[instrument(skip(self, ctx))]
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let user_data = command.user.clone();

            info!(
                "Slash command '{}' used by {}#{} with id {} on {:?}",
                command.data.name,
                user_data.name,
                user_data.discriminator,
                user_data.id.0,
                command.guild_id
            );

            let mut parse_success = false;

            let content = match command.data.name.as_str() {
                "create" => {
                    // run the command, get channel data as result and the string to send as a reply
                    let (parsing_status, command_reply) = create::run(&command.data.options);

                    // if channel data properly acquired, unlock struct value, write the channel data in hashmap and close
                    if let Ok(parsed) = parsing_status {
                        info!("Inserting parsed data: {parsed:#?}");
                        parse_success = true;
                        let parsed_data_lock = {
                            let read_data = ctx.data.read().await;
                            read_data.get::<ParsedData>().unwrap().clone()
                        };

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

            if parse_success {
                create::setup(&ctx, command, user_data).await;
            }
        }
    }
}
#[instrument]
pub async fn start_bot() {
    // initialize trace logging
    tracing_subscriber::fmt::init();
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

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
