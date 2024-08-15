use teloxide::{prelude::*, utils::command::BotCommands};

use crate::images_handler::{get_animal_image, Animal};
// Customize this struct with things from `shuttle_main` needed in `bind`,
// such as secrets or database connections
pub struct BotService {
    pub bot: Bot
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for BotService {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        self.start().await.expect("An error occured while using this bot.");
        Ok(())
    }
}

impl BotService {
    async fn start(&self) -> Result<(), shuttle_runtime::Error> {
        let bot = self.bot.clone();

        Command::repl(bot, answer).await;

        Ok(())
    }
}


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These are the listed commands:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "displays the startup message.")]
    Start,
    #[command(description = "displays the technical information, such as link to github repo, etc.")]
    About,
    #[command(
        description = "sends a picture of a random sable", 
        parse_with = "split"
    )]
    RandomSable,
    #[command(
        description = "sends a picture of a random samoyed", 
        parse_with = "split"
    )]
    RandomSamoyed,
    #[command(
        description = "sends a picture of a random snow leopard", 
        parse_with = "split"
    )]
    RandomSnowLeopard
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let result = match cmd {
        Command::Help => help(&bot, &msg).await,
        Command::Start => start(&bot, &msg).await,
        Command::About => about(&bot, &msg).await,
        Command::RandomSable => get_animal_image(&bot, &msg, Animal::Sable).await,
        Command::RandomSamoyed => get_animal_image(&bot, &msg, Animal::Samoyed).await,
        Command::RandomSnowLeopard => get_animal_image(&bot, &msg, Animal::SnowLeopard).await,
    };

    if let Err(_error) = result {
        let _ = bot.send_message(msg.chat.id, "Internal Server Error: {_error}. \nContact the owner to notify of the issue....").await;
    }

    Ok(())
}

async fn help(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

async fn start(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    let greeting_message = String::from("
        Goon day everynyan :3\n
        This is the SSSL Registry. Which stands for the:\n\n
        🐕 S - Samoyed\n
        🦦 S - Sable\n
        🐆 SL - Snow Leopard\n
        Registry, where you can fulfill your random SSSL needs!\n
        To get started, simply type /help !!!
    ");

    bot.send_message(msg.chat.id, greeting_message).await?;
    Ok(())
}

async fn about(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    let about_message = String::from("
        Want to see the source code? Find it in the github repo!\n
        https://github.com/JR-Bam/sssl-registry-tg-bot\n\n
        Made in rust, using the Teloxide crate, and deployed using Shuttle!
    ");

    bot.send_message(msg.chat.id, about_message).await?;
    Ok(())
}

