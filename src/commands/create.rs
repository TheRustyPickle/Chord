use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("create")
        .description("Command for creating channels")
        .dm_permission(false)
}

pub fn run(_options: &[CommandDataOption]) -> String {
    "Parsing will happen here".to_string()
}
