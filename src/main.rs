use chord::bot::Handler;
use chord::bot::{ParsedData, PermissionData};
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::prelude::*;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use std::collections::{HashMap, HashSet};
//use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::error;
//use tracing_subscriber::filter::LevelFilter;
//use tracing_subscriber::EnvFilter;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // initialize trace logging
    // if RUST_LOG=debug is passed, enable debug logging for current package only and accept all info and error logs
    // Otherwise, only info level logging is enabled
    /*let mut env_filter = EnvFilter::from_default_env();
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
    //tracing::subscriber::set_global_default(subscriber).unwrap();*/

    // get the bot token
    let token = secret_store
        .get("DISCORD_TOKEN")
        .expect("Discord token not found");

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
        std::process::exit(1);
    }

    Ok(client.into())
}
