<div align="center"><h1>Chord</h1></div>

<p align=center><a href="https://wakatime.com/badge/user/a56201d4-20a8-4c30-a6d7-2d8bb0e3d23c/project/638a131e-5bee-4e74-8c33-206a525af913"><img src="https://wakatime.com/badge/user/a56201d4-20a8-4c30-a6d7-2d8bb0e3d23c/project/638a131e-5bee-4e74-8c33-206a525af913.svg" alt="wakatime"></a></p>

A discord bot designed to create categories/channels in a guild with a cli like command. Primarily aimed at reducing some of the manual labor when creating lots of channels. Created with Serenity and deployed in Shuttle.

# Getting Started

**1. Run from Source Code:**

* Clone the repository
`
git clone https://github.com/TheRustyPickle/Chord
`
* Add your discord bot token to Secrets.toml file
* Install `cargo-shuttle` crate with
`
cargo install cargo-shuttle
`
  * Run locally with `cargo shuttle run`

  or

  * Deploy your own version of the bot with shuttle by following these commands:

    `cargo shuttle project start --idle-minutes 0`

    `cargo shuttle deploy`

* Add to a guild, check /help, /example and start creating channels with /create

**2. Try out the deployed version:**

* Add the bot to your server: [Bot Link](https://discord.com/api/oauth2/authorize?client_id=1041391133118451813&permissions=8&scope=bot)
* Try out various commands and see it in action

# Example Bot Commands

Here are some of the command examples and explanation that can be used with /create slash command on the bot.

**Command:** `-cat my category -p -r admin, member -ch general -ch memes`

**Explanation:** Create a private category 'my category' and channels 'general' and 'memes' inside the category where everyone with 'admin' and 'member' role will have access to the channels

**Command:** `-cat main chat -ch member-chat -ch admin-chat -r admin -p`

**Explanation:** Create a public category named 'main chat' where everyone will get access to 'member chat' and create a private channel inside the category named 'admin-chat' where only users with 'admin' role will get access
