use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("example")
        .description("Check examples of the accepted commands")
}

pub fn run(_options: &[CommandDataOption]) -> String {
    "Below are some example commands the bot can understand. Try each of them individually or combine them as one command to see the bot in action.
    
**Command:** -cat my category -p -r admin, member -ch general -ch memes -t text

**Explanation:** Creates a private category called 'my category' and channels 'general' and 'memes' within that category. Both 'admin' and 'member' roles will have access to these channels. The channels will be of type 'text'.

**Command:** -cat main chat -ch member-chat -ch admin-chat -r admin -p -t voice

**Explanation:** Creates a public category named 'main chat' where everyone has access to the 'member chat'. Additionally, it creates a private channel named 'admin-chat' within the category, accessible only to users with the 'admin' role. The channels will be of type 'voice'.

**Command:** -cat supporters chat -r supporter -p -ch general-supporter -r @everyone -ch supporter-announcement -t ann 

**Explanation:** Creates a private category named 'supporters' and grants access to users with the 'supporter' role. Within the category, it creates a channel called 'general-supporter' that is accessible to everyone. It also creates an announcement channel named 'supporter-announcement' within the category, which is exclusively accessible to users with the 'supporter' role. The 'supporter-announcement' channel is of type 'announcement'.".to_string()
}
