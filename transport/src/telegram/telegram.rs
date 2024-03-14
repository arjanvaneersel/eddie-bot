use crate::telegram::Config;
use eddie_lib::{origin::Origin, Call, Response};
use support::traits::{Dispatch, Get};
use teloxide::{prelude::*, utils::command::BotCommands};

/// Processor of requests coming from Telegram.
pub struct TelegramTransport<T: Config>(std::marker::PhantomData<T>);

impl<T: Config> TelegramTransport<T> {
    pub fn new() -> TelegramTransport<T> {
        TelegramTransport(std::marker::PhantomData)
    }

    async fn process(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
        let user = match msg.from() {
            Some(user) => user,
            None => return Ok(()),
        };
        let origin = Origin::Telegram(user.id.to_string());

        match cmd {
            Command::Help => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .reply_to_message_id(msg.id)
                    .await?;
            }
            Command::Info => match Call::<T>::Info.dispatch(origin) {
                Ok(Some(Response::Info(info))) => {
                    bot.send_message(msg.chat.id, info)
                        .reply_to_message_id(msg.id)
                        .await?;
                }
                Err(err) => {
                    bot.send_message(msg.chat.id, err.to_string())
                        .reply_to_message_id(msg.id)
                        .await?;
                }
                _ => {}
            },
            Command::Init => {
                if let Err(err) = Call::<T>::Init.dispatch(origin) {
                    bot.send_message(msg.chat.id, err.to_string())
                        .reply_to_message_id(msg.id)
                        .await?;
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
    Info,
    #[command(description = "initialize the bot")]
    Init,
    // #[command(description = "get some tokens from the faucet.")]
    // Faucet(String),
    // #[command(description = "handle a username.")]
    // Username(String),
    // #[command(description = "handle a username and an age.", parse_with = "split")]
    // UsernameAndAge { username: String, age: u8 },
}
