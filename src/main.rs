use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    log::info!("Starting my_telegram_bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, message: Message| async move {

    let chat_id = message.chat.id;

    if let Some(text) = message.text() {
        match text {
            "/start" => {
                bot.send_message(chat_id, "Добро пожаловать в бота!").await?;
            }
            "/help" => {
                bot.send_message(chat_id, "Список команд: /start, /help, /echo [текст]").await?;
            }
            _ if text.starts_with("/echo ") => {
                let echo_text = &text[6..]; // Удаляем '/echo '
                bot.send_message(chat_id, echo_text).await?;
            }
            _ => {
                bot.send_message(chat_id, "Извините, я не понимаю эту команду.").await?;
            }
        }
    }
    respond(())
    })
        .await;
}