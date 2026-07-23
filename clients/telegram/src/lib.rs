use teloxide::prelude::*;

pub async fn send_message(bot: Bot, message: &str) {
    let your_id = ChatId(0);
    bot.send_message(your_id, message).await.unwrap();
    ()
}
