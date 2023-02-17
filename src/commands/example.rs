use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("example")
        .description("Check examples of the accepted commands")
}

pub fn run(_options: &[CommandDataOption]) -> String {
    "1. -cat my category -p -r admin, member -ch general -ch memes

Explanation: Create a private category 'my category' where 'admin' and 'member' role will have access to the channels 'general' and 'memes'

2. -cat main chat -ch member-chat -ch admin-chat -r admin -p

Explanation: Create a public category named 'main chat' where everyone will get access to 'member chat' and create another private channel named 'admin-chat' where only 'admin' role will get access

3. -cat supporters chat -r supporter -p -ch general-supporter -r @everyone -ch supporter-announcement -t ann 

Explanation: Create a private category 'supporters' giving access to 'supporter' role holders. Create a 'general-supporter' channel where everyone will get access. Create an announcement channel named 'supporter-announcement' giving access to 'supporter' role holders

".to_string()
}
