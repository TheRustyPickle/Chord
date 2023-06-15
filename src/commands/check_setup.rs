use crate::utility::{get_locked_permissiondata, get_perm_list};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::prelude::*;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("check_setup")
        .description("Shows what setup was configured")
}

pub async fn run(_options: &[CommandDataOption], ctx: &Context, user_id: u64) -> String {
    {
        let locked_permissiondata = get_locked_permissiondata(ctx).await;
        let permissiondata = locked_permissiondata.read().await;

        // look for if the user already used /setup for permissions
        if permissiondata.contains_key(&user_id) {
            let mut reply_text = String::new();
            let perm_list = get_perm_list(); // returns a list of permissions used in /setup

            for (perm_type, perm) in &permissiondata[&user_id] {
                if perm.is_empty() {
                    reply_text.push_str(&format!("**{perm_type}**: None\n\n"));
                } else {
                    reply_text.push_str(&format!("**{perm_type}:** {perm}\n\n"));
                }
            }
            reply_text = reply_text.replace("public_allow", "Allowed for Public Channel");
            reply_text = reply_text.replace("private_allow", "Allowed for Private Channel");
            reply_text = reply_text.replace("public_deny", "Denied for Public Channel");
            reply_text = reply_text.replace("private_deny", "Denied for Private Channel");

            // perm list contains the serenity Permission and a readable version of the permission.
            // Replace serenity permission with a more readable text version
            for (perm_str, perm) in perm_list {
                reply_text = reply_text.replace(&format!("{perm}"), perm_str)
            }
            return reply_text;
        }
    }

    // default text if /setup was not used by the user
    "You have not setup any permissions. Use /setup to do so.".to_string()
}
