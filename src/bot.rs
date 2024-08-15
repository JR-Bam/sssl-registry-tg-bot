use teloxide::{prelude::*, utils::command::BotCommands};

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
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Start => bot.send_message(msg.chat.id, "Hello Everynyan : 3!\n Type /help to get started!").await?
    };

    Ok(())
}