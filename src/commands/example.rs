use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("example")
        .description("Check examples of the accepted commands")
}

pub fn run(_options: &[CommandDataOption]) -> String {
    "Below are some example commands the bot can understand. Try each of them individually or combine them as one command to see the bot in action.
    
**Command:** -cat my category -p -r admin, member -ch general -ch memes

**Explanation:** Create a private category 'my category' and channels 'general' and 'memes' inside the category where 'admin' and 'member' role will have access to the channels

**Command:** -cat main chat -ch member-chat -ch admin-chat -r admin -p

**Explanation:** Create a public category named 'main chat' where everyone will get access to 'member chat' and create a private channel inside the category named 'admin-chat' where only 'admin' role will get access

**Command:** -cat supporters chat -r supporter -p -ch general-supporter -r @everyone -ch supporter-announcement -t ann 

**Explanation:** Create a private category 'supporters' giving access to 'supporter' role holders. Create a 'general-supporter' channel inside the category where everyone will get access. Create an announcement channel named 'supporter-announcement' inside the category giving access to 'supporter' role holders".to_string()
}
