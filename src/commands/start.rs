use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("start")
        .description("This is a start command to navigate the bot")
}

pub fn run(_options: &[CommandDataOption]) -> String {
    "/start text will be placed here".to_string()
}
