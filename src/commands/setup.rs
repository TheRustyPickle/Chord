use crate::bot::PermissionData;
use crate::utility::normal_button;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::component::ButtonStyle;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption,
};
use serenity::model::user::User;
use serenity::model::Permissions;
use serenity::prelude::*;
use std::collections::HashMap;
use tracing::info;

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
    let perm_list = HashMap::from([
        ("view Channel", Permissions::VIEW_CHANNEL),
        ("Send Message", Permissions::SEND_MESSAGES),
        ("Manage Channel", Permissions::MANAGE_CHANNELS),
        ("Manage Roles", Permissions::MANAGE_ROLES),
        ("Attach Files", Permissions::ATTACH_FILES),
        ("Mention @everyone @here", Permissions::MENTION_EVERYONE),
        ("Manage Message", Permissions::MANAGE_MESSAGES),
        ("Read Message History", Permissions::READ_MESSAGE_HISTORY),
        ("Use Application Commands", Permissions::USE_SLASH_COMMANDS),
    ]);

    let mut public_allow = Permissions::empty();
    let mut public_deny = Permissions::empty();
    let mut private_allow = Permissions::empty();
    let mut private_deny = Permissions::empty();

    for (permission, perm) in perm_list {
        let followup_mess = command
            .create_followup_message(&ctx.http, |message| {
                message
                    .content(format!(
                        "Answer the following question\n\nWhat do you want to do with **{permission}**?"
                    ))
                    .components(|c| {
                        c.create_action_row(|row| {
                            row.add_button(normal_button("Allow For Public", ButtonStyle::Primary));
                            row.add_button(normal_button("Deny For Public", ButtonStyle::Primary))
                        });
                        c.create_action_row(|row| {
                            row.add_button(normal_button("Allow For Private", ButtonStyle::Primary));
                            row.add_button(normal_button("Deny For Private", ButtonStyle::Primary))
                        });
                        c.create_action_row(|row| {
                            row.add_button(normal_button("Allow Public, Deny Private", ButtonStyle::Primary));
                            row.add_button(normal_button("Deny Public Allow Private", ButtonStyle::Primary))
                        });
                        c.create_action_row(|row| {
                            row.add_button(normal_button("Allow For Both", ButtonStyle::Primary));
                            row.add_button(normal_button("Deny For Both", ButtonStyle::Primary))
                        });
                        c.create_action_row(|row| {
                            row.add_button(normal_button(
                                "Keep Default/Unchanged",
                                ButtonStyle::Primary,
                            ))
                        })
                    })
            })
            .await
            .unwrap();

        let followup_reply = followup_mess.await_component_interaction(&ctx).await;
        match followup_reply {
            Some(reply) => {
                let reply_str = reply.data.custom_id.as_str();

                info!(
                    "Selected '{reply_str}' for '{permission}' by {}#{} with id {} on guild {:?} {}",
                    user_data.name,
                    user_data.discriminator,
                    user_data.id.0,
                    command.guild_id.unwrap().name(&ctx),
                    command.guild_id.unwrap()
                );
                followup_mess.delete(&ctx.http).await.unwrap();
                match reply_str {
                    "Allow For Public" => public_allow = public_allow.union(perm),
                    "Allow For Private" => private_allow = private_allow.union(perm),
                    "Deny For Public" => public_deny = public_deny.union(perm),
                    "Deny For Private" => private_deny = private_deny.union(perm),
                    "Allow Public, Deny Private" => {
                        public_allow = public_allow.union(perm);
                        private_deny = private_deny.union(perm);
                    }
                    "Deny Public Allow Private" => {
                        public_deny = public_deny.union(perm);
                        private_allow = private_allow.union(perm);
                    }
                    "Allow For Both" => {
                        public_allow = public_allow.union(perm);
                        private_allow = private_allow.union(perm);
                    }
                    "Deny For Both" => {
                        private_allow = private_allow.union(perm);
                        private_deny = private_deny.union(perm);
                    }

                    _ => {}
                }
            }
            None => {}
        }
    }
}
