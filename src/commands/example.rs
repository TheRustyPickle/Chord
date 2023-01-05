use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("example").description("This is a help command")
}

pub fn run(_options: &[CommandDataOption]) -> String {
    "Examples to be placed here".to_string()
}
