use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{CommandDataOption, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use crate::parser::parse_input;
use crate::channel_data::ChannelInfo;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("create")
        .description("Command for creating channels")
        //.dm_permission(false)
        .create_option(|option| {
            option
            .name("string")
            .description("command list for channel creation")
            .kind(CommandOptionType::String)
            .required(true)
        })
}

pub fn run(options: &[CommandDataOption]) -> (Vec<ChannelInfo>, String) {
    let resolved =  options.get(0).expect("Some value").resolved.as_ref().expect("Some value");
    if let CommandDataOptionValue::String(value) = resolved {
        let parsed_data = parse_input(value.to_string()).unwrap();
        let mut detected_text = format!("These data were detected\n");

        for (key, value) in parsed_data.iter() {
            match key {
                &"category" => detected_text.push_str(&format!("Category: {}\n", value[0])),
                &"channels" => {
                    detected_text.push_str("Channels:");
                    for i in &parsed_data[key] {
                        detected_text.push_str(&format!(" {i},"));
                    }
                    detected_text.pop().unwrap();
                    detected_text.push_str("\n")
                }
                &"roles" => {
                    detected_text.push_str("Roles:");
                    for i in &parsed_data[key] {
                        detected_text.push_str(&format!(" {i},"));
                    }
                    detected_text.pop().unwrap();
                    detected_text.push_str("\n")
                }
                _ => {}
            }
        }

        (vec![ChannelInfo::default()], detected_text)
    } else {
        (vec![ChannelInfo::default()], "No value was given. Parsing will happen here".to_string())
    }
    
}
