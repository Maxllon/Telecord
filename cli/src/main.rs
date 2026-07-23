use telecord_telegram::send_message;
use teloxide::prelude::*;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    let bot = Bot::from_env();
    send_message(bot, "Hi from Telecord!").await;
    Ok(())
}
