use crate::{create, help, start};
use crate::channel_data::{ChannelInfo};

use serenity::async_trait;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::collections::HashSet;
use std::env;

struct Handler {
    parsed_data: Vec<ChannelInfo>
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, _ready: Ready) {
        println!("I am ready to receive");
        let status = Command::set_global_application_commands(&context.http, |commands| {
            commands
                .create_application_command(|command| create::register(command))
                .create_application_command(|command| help::register(command))
                .create_application_command(|command| start::register(command))
        })
        .await;

        if let Err(e) = status {
            println!("{e}");
            std::process::exit(1)
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {

            let content = match command.data.name.as_str() {
                "create" => {
                    let (parsing_status, command_reply) = create::run(&command.data.options);
                    command_reply
                },
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

    let default_handler = Handler {
        parsed_data: Vec::new()
    };

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(default_handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}