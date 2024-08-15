use bot::BotService;
use shuttle_runtime::SecretStore;
use teloxide::Bot;

mod bot;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore
) -> Result<BotService, shuttle_runtime::Error> {

    let teloxide_key = secrets
        .get("TELOXIDE_TOKEN")
        .expect("Teloxide key not found.");

    Ok(BotService {
        bot: Bot::new(teloxide_key)
    })
}

