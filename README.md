<div align="center"><h1>Chord</h1></div>

<p align=center><a href="https://wakatime.com/badge/user/a56201d4-20a8-4c30-a6d7-2d8bb0e3d23c/project/638a131e-5bee-4e74-8c33-206a525af913"><img src="https://wakatime.com/badge/user/a56201d4-20a8-4c30-a6d7-2d8bb0e3d23c/project/638a131e-5bee-4e74-8c33-206a525af913.svg" alt="wakatime"></a></p>

A Discord bot designed to create categories and channels in a guild with a CLI-like command. It is primarily aimed at reducing manual labor when creating multiple channels. Chord is created with Serenity and deployed using Shuttle.

<h2>Getting Started</h2>

**1. Run from Source Code:**

* Clone the repository
`
git clone https://github.com/TheRustyPickle/Chord
`
* Add your discord bot token to `Secrets.toml` file
* Install the `cargo-shuttle` crate:
`
cargo install cargo-shuttle
`
  * Run locally with `cargo shuttle run`

  or

  * Deploy your own version of the bot using Shuttle by following these commands:

    `cargo shuttle project start --idle-minutes 0`

    `cargo shuttle deploy`

* Add the bot to a guild, then use /help, /example and start creating channels with /create

**2. Try the Deployed Version:**

* Add the bot to your server using [this link](https://discord.com/api/oauth2/authorize?client_id=1041391133118451813&permissions=8&scope=bot)
* Explore various commands and see it in action.

<h2>Example Bot Commands</h2>

Here are a few examples of commands that can be used with the /create slash command in the bot:

**Command:** `-cat my category -p -r admin, member -ch general -ch memes`

**Explanation:** Create a private category 'my category' and channels 'general' and 'memes' inside the category where everyone with 'admin' and 'member' role will have access to the channels

**Command:** `-cat main chat -ch member-chat -ch admin-chat -r admin -p`

**Explanation:** Create a public category named 'main chat' where everyone will get access to 'member chat' and create a private channel inside the category named 'admin-chat' where only users with 'admin' role will get access

**Command:** `-cat supporters chat -r supporter -p -ch general-supporter -r @everyone -ch supporter-announcement -t ann`

**Explanation:** Creates a private category named 'supporters' and grants access to users with the 'supporter' role. Within the category, it creates a channel called 'general-supporter' that is accessible to everyone. It also creates an announcement channel named 'supporter-announcement' within the category, which is exclusively accessible to users with the 'supporter' role. The 'supporter-announcement' channel is of type 'announcement'

<h2>Setup Command</h2>

The /setup command can be used to quickly set initial permissions for channels and categories when using /create. It serves as a starting point for permissions setup and is intended to be modified later as necessary. To use it, type /setup and follow the prompts to configure the permission settings for private and public channels.

<h2>Feedback & Bug Reports</h2>

For any feedback, improvement suggestions or bugs please [open an issue](https://github.com/TheRustyPickle/Chord/issues/new)
