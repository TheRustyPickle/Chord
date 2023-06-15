use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("start")
        .description("Shows relevant command of the bot")
}

pub fn run(_options: &[CommandDataOption]) -> String {
    "This bot is intended to create Categories and Channel in a discord guild, fast, using CLI like messages. It should be used as a faster way to create multiple channels and categories using a small message and later modified to the liking and the necessities. It aims to reduce the hassle when creating a guild from the scratch.
    
Following commands are available
    
**/create:** Accepts a string that is parsed for creating categories and channels
**/setup:** Setup permissions for guilds
**/check_setup:** Shows the permission that was set
**/help:** Shows how to use this bot and parameters that are recognized
**/example:** Get some examples of what kind of commands are recognized".to_string()
}
