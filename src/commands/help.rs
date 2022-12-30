use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("help").description("This is a help command")
}

pub fn run(_options: &[CommandDataOption]) -> String {
    "**How to use /create command**
    
Examples:

-cat first category -r members, verified -ch channel-1 -t ann -r admin, verified | channel-2 -p

-cat third category -r cm, mods, administrator, -p -ch channel-3 -t text -r discord, bots | channel-4

Parameters:

-cat: Highlights a category

-r: Highlights an existing Role/s inside the guild. Optional

-p: Recognizes a Channel or a Category to be as Private. Optional

-ch: Highlights a Channel

-t: Highlights the Channel type. Optional

Rules:

-cat: 
    - Category name can be as long as necessary
    - If no -ch is given after -cat, the category will not be created
    - If -p or -r are passed after -cat, any -ch after it will follow the rules of the category
    - Supported values: -r, -p, -ch
-r:
    - Can be passed both after -cat or -ch. Example: -cat category -r role -ch channel-1 -r role 2
    - If -r is passed after -ch, it will overwrite -r inside the -cat
    - @everyone as a role can be passed to overwrite -p flag of a -cat
    - Multiple roles can be given after -r. Must be separated by comma + space. Example: -ch channel -r role 1, role 2, role 3
-p:
    - If -p is passed after -cat, the Category will be private
    - If -p is passed after -ch, the Channel will private
    - If -r is passed alongside -p, only the given role will get access
-ch:
    - Channel name can be as long as necessary
    - Channel names must be separated by | + space. Example: -ch channel | channel-2
    - If -ch is placed after -cat, it will follow -cat rules
    - If -cat is not found, it will be created without a category
    - Supported values: -r, -p, -t
-t:
    - Should be used after -ch
    - 'text' = Text channel. Default value
    - 'voice' = Voice channel
    - 'ann' = Announcement/News Channel
    - If not is passed, Text channel.

Recommended Steps:

    - Create the necessary roles before using /create
    - Line breaks are supported
    - Divide each category with channels with line breaks to simplify".to_string()
}
