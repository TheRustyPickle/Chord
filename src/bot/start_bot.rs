use crate::bot::ChannelInfo;
use crate::{create, help, start};
use serenity::async_trait;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;

struct ParsedData;

impl TypeMapKey for ParsedData {
    type Value = Arc<RwLock<HashMap<u64, Vec<ChannelInfo>>>>;
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
            println!("Client crashed. Exiting. Reason: {e}");
            std::process::exit(1)
        }
        println!("I am ready to receive");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "create" => {
                    let (parsing_status, command_reply) = create::run(&command.data.options);

                    let user_id = ctx.cache.current_user().id.0;

                    let parsed_data_lock = {
                        let read_data = ctx.data.read().await;
                        read_data.get::<ParsedData>().unwrap().clone()
                    };

                    {
                        let mut parsed_data = parsed_data_lock.write().await;
                        parsed_data.insert(user_id, parsing_status);
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
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

pub async fn start_bot() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new(&token);

    // We will fetch your bot's owners and id
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
        println!("Client error: {:?}", why);
    }
}
