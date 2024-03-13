use crate::telegram::Config;
use eddie_lib::{Call, Response};
use support::traits::{Dispatch, Get};
use teloxide::{prelude::*, utils::command::BotCommands};

/// Processor of requests coming from Telegram.
pub struct TelegramTransport<T: Config>(std::marker::PhantomData<T>);

impl<T: Config> TelegramTransport<T> {
    pub fn new() -> TelegramTransport<T> {
        TelegramTransport(std::marker::PhantomData)
    }

    async fn process(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
        match cmd {
            Command::Help => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .reply_to_message_id(msg.id)
                    .await?;
            }
            Command::Version => {
                let response = (Call::<T>::Version).dispatch(());
                log::info!("Received bot response: {:?}", response);
                match response {
                    Ok(Some(Response::Version(version))) => {
                        if let Err(why) = bot
                            .send_message(msg.chat.id, version)
                            .reply_to_message_id(msg.id)
                            .await
                        {
                            log::error!("Couldn't send answer to Telegram: {:?}", why);
                        }
                    }
                    Ok(None) => {}
                    Err(err) => {
                        if let Err(why) = bot
                            .send_message(msg.chat.id, err.to_string())
                            .reply_to_message_id(msg.id)
                            .await
                        {
                            log::error!("Couldn't send answer to Telegram: {:?}", why);
                        }
                        // Err(err)?
                    }
                }
            }
        }
        // Command::Username(username) => {
        //     bot.send_message(msg.chat.id, format!("Your username is @{username}."))
        //         .await?
        // }
        // Command::UsernameAndAge { username, age } => {
        //     bot.send_message(
        //         msg.chat.id,
        //         format!("Your username is @{username} and age is {age}."),
        //     )
        //     .await?
        // }
        // };

        Ok(())
    }

    pub async fn serve(&self) {
        log::info!("Starting Telegram bot");

        let bot = Bot::new(<T as Config>::Token::get());
        Command::repl(bot, Self::process).await;
    }
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "get help")]
    Help,
    #[command(description = "get the bot version")]
    Version,
    // #[command(description = "get some tokens from the faucet.")]
    // Faucet(String),
    // #[command(description = "handle a username.")]
    // Username(String),
    // #[command(description = "handle a username and an age.", parse_with = "split")]
    // UsernameAndAge { username: String, age: u8 },
}
