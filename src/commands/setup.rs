use crate::utility::{get_guild_name, get_locked_permissiondata, get_perm_list, normal_button};
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::component::ButtonStyle;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption,
};
use serenity::model::user::User;
use serenity::model::Permissions;
use serenity::prelude::*;
use serenity::Error;
use std::collections::HashMap;
use tracing::info;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("setup")
        .description("Setup the default permissions for channels")
        .dm_permission(false)
}

pub fn run(_options: &[CommandDataOption]) -> String {
    "A series of messages will be sent. Click on the appropriate button for permission confirmation"
        .to_string()
}

pub async fn setup(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    user_data: User,
) -> Result<(), Error> {
    // get the permission list
    let perm_list = get_perm_list();

    let user_id = user_data.id.0;

    let mut public_allow = Permissions::empty();
    let mut public_deny = Permissions::empty();
    let mut private_allow = Permissions::empty();
    let mut private_deny = Permissions::empty();

    // loop through all permissions -> send a message -> wait for a reply -> delete message
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
                            row.add_button(normal_button("Deny Public, Allow Private", ButtonStyle::Primary))
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
            .await?;

        let followup_reply = followup_mess.await_component_interaction(ctx).await;
        if let Some(reply) = followup_reply {
            let reply_str = reply.data.custom_id.as_str();

            let guild_id = command.guild_id.unwrap_or(0.into());

            info!(
                "Selected '{reply_str}' for '{permission}' by {} with id {} on guild {:?} {}",
                user_data.name,
                user_id,
                get_guild_name(ctx, guild_id).await,
                guild_id
            );
            followup_mess.delete(&ctx.http).await?;
            match reply_str {
                "Allow For Public" => public_allow = public_allow.union(perm),
                "Allow For Private" => private_allow = private_allow.union(perm),
                "Deny For Public" => public_deny = public_deny.union(perm),
                "Deny For Private" => private_deny = private_deny.union(perm),
                "Allow Public, Deny Private" => {
                    public_allow = public_allow.union(perm);
                    private_deny = private_deny.union(perm);
                }
                "Deny Public, Allow Private" => {
                    public_deny = public_deny.union(perm);
                    private_allow = private_allow.union(perm);
                }
                "Allow For Both" => {
                    public_allow = public_allow.union(perm);
                    private_allow = private_allow.union(perm);
                }
                "Deny For Both" => {
                    public_deny = public_deny.union(perm);
                    private_deny = private_deny.union(perm);
                }

                _ => {}
            }
        }
    }

    let locked_permission = get_locked_permissiondata(ctx).await;

    {
        // save the permission data that was collected
        let mut saved_permissions = locked_permission.write().await;
        saved_permissions.insert(
            user_id,
            HashMap::from([
                ("public_allow".to_string(), public_allow),
                ("public_deny".to_string(), public_deny),
                ("private_allow".to_string(), private_allow),
                ("private_deny".to_string(), private_deny),
            ]),
        );
    }

    command
        .edit_original_interaction_response(&ctx, |resp| {
            resp.content("Permission setup complete. This will be persisted across guilds.")
        })
        .await?;
    Ok(())
}
