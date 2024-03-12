use crate::discord::{helpers, Config};
use eddie_lib::{Call, Response};
use support::traits::{Dispatch, Get};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

/// Processor of requests coming from Discord.
pub struct DiscordTransport<T: Config>(std::marker::PhantomData<T>);

impl<T: Config> DiscordTransport<T> {
    pub fn new() -> DiscordTransport<T> {
        DiscordTransport(std::marker::PhantomData)
    }

    pub fn processor(command: String, _args: Vec<String>) -> Result<String, String> {
        let command: &str = &command.to_lowercase();
        let call = match command {
            "version" => Call::<T>::Version,
            _ => return Err(format!("The command {:?} is unknown to be.", command)),
        };

        match call.dispatch(()) {
            Ok(Some(Response::Version(version))) => Ok(version),
            Ok(None) => Ok(String::new()),
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn serve(&self) -> Result<(), SerenityError> {
        log::info!("Starting Discord bot");

        // Set gateway intents, which decides what events the bot will be notified about
        // More info: https://github.com/serenity-rs/serenity
        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;

        let mut client = Client::builder(T::Token::get(), intents)
            .event_handler(Handler::new(Self::processor))
            .await?;

        client.start().await
    }
}

/// Handler for Discord messages
struct Handler {
    /// Processor is a function that will process a command.
    //
    // A callback function is used, because EventHandler doesn't like PhantomData used in DiscordTransport,
    // therefore EventHandler couldn't be implemented on DiscordTransport.
    // TODO: Find a better way than working with callbacks.
    pub processor: fn(String, Vec<String>) -> Result<String, String>,
}

impl Handler {
    pub fn new(processor: fn(String, Vec<String>) -> Result<String, String>) -> Self {
        Self { processor }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore messages from the bot itself.
        if msg.author.id == ctx.cache.current_user().id {
            return;
        }

        log::debug!("Received Discord message: {:?}", &msg);

        let (command, arg) = helpers::split_with_quotes(msg.content);
        match (self.processor)(command, arg) {
            Ok(reply) => {
                if reply != "" {
                    // Reply with the response.
                    if let Err(why) = msg.channel_id.say(&ctx.http, reply).await {
                        log::error!("Error sending Discord message: {:?}", why)
                    }
                }
            }
            Err(err) => {
                // Log the error
                log::error!("Error processing Discord message: {:?}", err);

                // Reply with the error.
                if let Err(why) = msg.channel_id.say(&ctx.http, err).await {
                    log::error!("Error sending Discord message: {:?}", why)
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        log::info!("Connected to Discord as {}", ready.user.name);
    }
}
