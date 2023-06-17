use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("help").description("This is a help command")
}

pub fn run(_options: &[CommandDataOption]) -> String {
    "**How to Use the /create Command**

**Parameters:**

**-cat:** Specifies the category name
    • Category name can be as long as necessary
    • Multiple -cat in one command are supported
    • If no -ch is given after -cat, the category will not be created
    • If -p or -r are passed after -cat, any -ch after it will follow the rules of the category
    • Supported values: -r, -p, -ch

**-ch:** Specifies the channel name
    • Channel name can be as long as necessary
    • Use -ch multiple times to create more channels. Example: `-ch channel-1 -p -ch another channel`
    • If -ch is placed after -cat, it will follow -cat rules
    • If -cat is not found, it will be created without a category
    • Supported values: -r, -p, -t

**-r:** Specifies one or more roles
    • If -r is passed after -ch, it will overwrite the roles specified in -cat
    • @everyone as a role can be passed to overwrite -p flag of a -cat
    • Multiple roles can be given after -r. Must be separated by comma and space. Example: `-ch channel -r @everyone, role 2, role 3`

**-p:** Specifies whether the category or channel is private
    • If -p is passed after -cat, the Category will be private
    • If -p is passed after -ch, the Channel will private
    • If -r is passed alongside -p, only the given roles will get access

**-t:**
    • Can be used after -ch
    • Values: 'text' for a text channel, 'voice' for a voice channel, 'ann' for an announcement channel
    • Example: `-ch channel -t voice`
    • If not is passed, defaults to Text channel

**Recommended Steps:**
    • Create the necessary roles before using /create
    • Before using the /create, separate each command with a line break for better readability and understanding. 
    Example:`
    -cat one ...
    -cat another ...`
    • See /examples and try them".to_string()
}
