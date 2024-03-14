use crate::telegram::Config;
use eddie_lib::{origin::Origin, Call, Response};
use support::traits::{Dispatch, Get};
use teloxide::{prelude::*, utils::command::BotCommands};

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
    #[command(description = "set a bot administrator", parse_with = "split")]
    SetAdmin { admin_origin: String, remove: bool },
    #[command(description = "register a channel as a faucet")]
    RegisterFaucetChannel,
    #[command(description = "get your user id")]
    UserId,
    // #[command(description = "get some tokens from the faucet.")]
    // Faucet(String),
    // #[command(description = "handle a username.")]
    // Username(String),
    // #[command(description = "handle a username and an age.", parse_with = "split")]
    // UsernameAndAge { username: String, age: u8 },
}

/// Processor of requests coming from Telegram.
pub struct TelegramTransport<T: Config>(std::marker::PhantomData<T>);

impl<T: Config> TelegramTransport<T> {
    pub fn new() -> TelegramTransport<T> {
        TelegramTransport(std::marker::PhantomData)
    }

    async fn process(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
        let sender = match msg.from() {
            Some(user) => user,
            None => return Ok(()),
        };
        let origin = Origin::Telegram(sender.id.to_string());

        match cmd {
            Command::Help => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .reply_to_message_id(msg.id)
                    .await?;
            }
            Command::Info => match Call::<T>::Info.dispatch(origin) {
                Ok(Some(Response::Reply(info))) => {
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
            Command::SetAdmin {
                admin_origin,
                remove,
            } => {
                if let Ok(admin_origin) = Origin::try_from(admin_origin) {
                    match Call::<T>::SetAdmin(admin_origin, remove).dispatch(origin) {
                        Ok(Some(Response::Reply(reply))) => {
                            bot.send_message(msg.chat.id, reply)
                                .reply_to_message_id(msg.id)
                                .await?;
                        }
                        Err(err) => {
                            bot.send_message(msg.chat.id, err.to_string())
                                .reply_to_message_id(msg.id)
                                .await?;
                        }
                        _ => {}
                    }
                } else {
                    bot.send_message(msg.chat.id, "Invalid origin for admin.")
                        .reply_to_message_id(msg.id)
                        .await?;
                }
            }
            Command::RegisterFaucetChannel => {
                let channel = Origin::Telegram(msg.chat.id.to_string());
                if let Err(err) = Call::<T>::RegisterFaucetChannel(channel).dispatch(origin) {
                    bot.send_message(msg.chat.id, err.to_string())
                        .reply_to_message_id(msg.id)
                        .await?;
                }
            }
            Command::UserId => {
                bot.send_message(msg.chat.id, format!("Your userid is {}.", sender.id))
                    .reply_to_message_id(msg.id)
                    .await?;
            }
        }

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
