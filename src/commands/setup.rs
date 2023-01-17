use crate::start_bot::{normal_button, ParsedData};
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::component::ButtonStyle;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption,
};
use serenity::model::user::User;
use serenity::model::Permissions;
use serenity::prelude::*;
use tracing::{info, instrument};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("setup")
        .description("Setup the default permissions for channels")
}

pub fn run(_options: &[CommandDataOption]) -> String {
    "A series of messages will be sent. Click on the appropriate button for permission confirmation"
        .to_string()
}

pub async fn setup(ctx: &Context, command: ApplicationCommandInteraction, user_data: User) {
    let permission_list = [
        "view channel",
        "send message",
        "manage channel",
        "read message history",
    ];
    for permission in permission_list {
        let followup_mess = command
            .create_followup_message(&ctx.http, |message| {
                message
                    .content(format!(
                        "Answer the following question\n\nTurn on **{permission}**?"
                    ))
                    .components(|c| {
                        c.create_action_row(|row| {
                            row.add_button(normal_button("Yes For Public", ButtonStyle::Primary));
                            row.add_button(normal_button("Yes For Private", ButtonStyle::Primary))
                        });
                        c.create_action_row(|row| {
                            row.add_button(normal_button("Yes For Both", ButtonStyle::Primary));
                            row.add_button(normal_button("No For Both", ButtonStyle::Primary))
                        })
                    })
            })
            .await
            .unwrap();

        let followup_reply = followup_mess.await_component_interaction(&ctx).await;
        match followup_reply {
            Some(reply) => {
                let reply_str = reply.data.custom_id.as_str();

                info!("Replied {reply_str} for {permission}");

                match reply_str {
                    _ => {
                        followup_mess.delete(&ctx.http).await.unwrap();
                    }
                }
            },
            None => {}
        }
    }
}
