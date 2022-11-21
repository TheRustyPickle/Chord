use crate::bot::ChannelInfo;
use crate::parse::{parse_to_channel, parse_to_text};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("create")
        .description("Command for creating channels")
        .dm_permission(false)
        .create_option(|option| {
            option
                .name("string")
                .description("command list for channel creation")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

pub fn run(options: &[CommandDataOption]) -> (Vec<ChannelInfo>, String) {
    let resolved = options
        .get(0)
        .expect("Some value")
        .resolved
        .as_ref()
        .expect("Some value");
    if let CommandDataOptionValue::String(value) = resolved {
        let reply_string = parse_to_text(value.to_string());
        if let Ok(data) = parse_to_channel(value.to_string()) {
            (data, reply_string)
        } else {
            (vec![ChannelInfo::default()], reply_string)
        }
    } else {
        (
            vec![ChannelInfo::default()],
            "No value was given. Parsing will happen here".to_string(),
        )
    }
}
