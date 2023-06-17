use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("start")
        .description("Shows relevant command of the bot")
}

pub fn run(_options: &[CommandDataOption]) -> String {
    "Chord is a Discord bot designed to create categories and channels in a guild with a CLI-like command. It is primarily aimed at reducing manual labor when creating multiple channels. 
    
Following commands are available
    
**/create:** Accepts a command string for creating categories and channels
**/setup:** Setup permissions for guilds
**/check_setup:** Shows the permission that was set
**/help:** Shows the parameters that are recognized
**/example:** Get some examples of what kind of commands are recognized

Source code: <https://github.com/TheRustyPickle/Chord>

Facing any bugs, issues or have a feedback? Create an issue on github.
".to_string()
}
