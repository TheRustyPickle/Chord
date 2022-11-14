use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("help").description("This is a help command")
}

pub fn run(_options: &[CommandDataOption]) -> String {
    "Help text is under construction".to_string()
}
