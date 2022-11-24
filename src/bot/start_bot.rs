use crate::bot::ChannelInfo;
use crate::{create, help, start};
use serenity::async_trait;
use serenity::builder::CreateButton;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::application::command::Command;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, debug, instrument};

struct ParsedData;

impl TypeMapKey for ParsedData {
    type Value = Arc<RwLock<HashMap<u64, Vec<ChannelInfo>>>>;
}

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
    async fn ready(&self, context: Context, _ready: Ready) {
        let status = Command::set_global_application_commands(&context.http, |commands| {
            commands
                .create_application_command(|command| create::register(command))
                .create_application_command(|command| help::register(command))
                .create_application_command(|command| start::register(command))
        })
        .await;

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
                    let (parsing_status, command_reply) = create::run(&command.data.options);
                    if let Ok(parsed) = parsing_status {
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
                "help" => help::run(&command.data.options),
                "start" => start::run(&command.data.options),
                _ => "Command not found".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            if command.data.name == "create" && parse_success {
                                message.content(content).components(|c| {
                                    c.create_action_row(|row| {
                                        row.add_button(normal_button(
                                            "Accept",
                                            ButtonStyle::Primary,
                                        ));
                                        row.add_button(normal_button("Reject", ButtonStyle::Danger))
                                    })
                                })
                            } else {
                                message.content(content)
                            }
                        })
                })
                .await
            {
                debug!("Cannot respond to slash command: {}", why);
            }
        }
    }
}
#[instrument]
pub async fn start_bot() {
    tracing_subscriber::fmt::init();
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

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ParsedData>(Arc::new(RwLock::new(HashMap::new())));
    }

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
