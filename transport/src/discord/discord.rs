use crate::discord::{commands, Config};
use eddie_lib::{Call, Config as BotConfig, Response};
use poise::{serenity_prelude as serenity, serenity_prelude::ClientBuilder};
use std::{sync::Arc, time::Duration};
use support::traits::{
    dispatch::{DispatchError, DispatchResult},
    Dispatch, Get,
};

pub struct Data<T: Config + BotConfig>(std::marker::PhantomData<T>);

// Types used by all command functions
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a, T> = poise::Context<'a, Data<T>, Error>;

async fn on_error<T: Config + BotConfig>(error: poise::FrameworkError<'_, Data<T>, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            log::error!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                log::error!("Error while handling error: {}", e)
            }
        }
    }
}

/// Processor of requests coming from Discord.
pub struct DiscordTransport<T: Config>(std::marker::PhantomData<T>);

impl<T: Config> DiscordTransport<T> {
    pub fn new() -> DiscordTransport<T> {
        DiscordTransport(std::marker::PhantomData)
    }

    pub async fn serve(&self) -> Result<(), serenity::Error> {
        log::info!("Starting Discord bot");

        // let cmds = commands::Commands::new();

        // FrameworkOptions contains all of poise's configuration option in one struct
        // Every option can be omitted to use its default value
        let options = poise::FrameworkOptions {
            commands: vec![
                commands::help::<T>(),
                commands::version::<T>(),
                commands::vote::<T>(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("$".into()),
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                    Duration::from_secs(3600),
                ))),
                additional_prefixes: vec![
                    poise::Prefix::Literal("hey bot"),
                    poise::Prefix::Literal("hey bot,"),
                ],
                ..Default::default()
            },
            // The global error handler for all error cases that may occur
            on_error: |error| Box::pin(on_error(error)),
            // This code is run before every command
            pre_command: |ctx| {
                Box::pin(async move {
                    log::info!("Executing command {}...", ctx.command().qualified_name);
                })
            },
            // This code is run after a command if it was successful (returned Ok)
            post_command: |ctx| {
                Box::pin(async move {
                    log::info!("Executed command {}!", ctx.command().qualified_name);
                })
            },
            // Every command invocation must pass this check to continue execution
            // command_check: Some(|ctx| {
            //     Box::pin(async move {
            //         if ctx.author().id == 123456789 {
            //             return Ok(false);
            //         }
            //         Ok(true)
            //     })
            // }),
            // Enforce command checks even for owners (enforced by default)
            // Set to true to bypass checks, which is useful for testing
            skip_checks_for_owners: false,
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(async move {
                    log::info!(
                        "Got an event in event handler: {:?}",
                        event.snake_case_name()
                    );
                    Ok(())
                })
            },
            ..Default::default()
        };

        let framework = poise::Framework::builder()
            .setup(move |ctx, _ready, framework| {
                Box::pin(async move {
                    log::info!("Logged in as {}", _ready.user.name);
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok(Data(std::marker::PhantomData))
                })
            })
            .options(options)
            .build();

        // Set gateway intents, which decides what events the bot will be notified about
        // More info: https://github.com/serenity-rs/serenity
        let intents =
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;
        // | GatewayIntents::GUILD_MESSAGES
        // | GatewayIntents::DIRECT_MESSAGES

        let mut client = ClientBuilder::new(<T as Config>::Token::get(), intents)
            .framework(framework)
            .await?;

        client.start().await
    }
}

// impl<T: Config> DiscordTransport<T> {
//     pub fn new() -> DiscordTransport<T> {
//         DiscordTransport(std::marker::PhantomData)
//     }

//     pub fn processor(command: String, _args: Vec<String>) -> Result<String, String> {
//         let command: &str = &command.to_lowercase();
//         let call = match command {
//             "version" => Call::<T>::Version,
//             _ => return Err(format!("The command {:?} is unknown to be.", command)),
//         };

//         match call.dispatch(()) {
//             Ok(Some(Response::Version(version))) => Ok(version),
//             Ok(None) => Ok(String::new()),
//             Err(err) => Err(err.to_string()),
//         }
//     }

//     pub async fn serve(&self) -> Result<(), SerenityError> {
//         log::info!("Starting Discord bot");

//         // Set gateway intents, which decides what events the bot will be notified about
//         // More info: https://github.com/serenity-rs/serenity
//         let intents = GatewayIntents::GUILD_MESSAGES
//             | GatewayIntents::DIRECT_MESSAGES
//             | GatewayIntents::MESSAGE_CONTENT;

//         let mut client = Client::builder(T::Token::get(), intents)
//             .event_handler(Handler::new(Self::processor))
//             .await?;

//         client.start().await
//     }
// }

// /// Handler for Discord messages
// struct Handler {
//     /// Processor is a function that will process a command.
//     //
//     // A callback function is used, because EventHandler doesn't like PhantomData used in DiscordTransport,
//     // therefore EventHandler couldn't be implemented on DiscordTransport.
//     // TODO: Find a better way than working with callbacks.
//     pub processor: fn(String, Vec<String>) -> Result<String, String>,
// }

// impl Handler {
//     pub fn new(processor: fn(String, Vec<String>) -> Result<String, String>) -> Self {
//         Self { processor }
//     }
// }

// #[async_trait]
// impl EventHandler for Handler {
//     async fn message(&self, ctx: Context, msg: Message) {
//         // Ignore messages from the bot itself.
//         if msg.author.id == ctx.cache.current_user().id {
//             return;
//         }

//         log::debug!("Received Discord message: {:?}", &msg);

//         let (command, arg) = helpers::split_with_quotes(msg.content);
//         match (self.processor)(command, arg) {
//             Ok(reply) => {
//                 if reply != "" {
//                     // Reply with the response.
//                     if let Err(why) = msg.channel_id.say(&ctx.http, reply).await {
//                         log::error!("Error sending Discord message: {:?}", why)
//                     }
//                 }
//             }
//             Err(err) => {
//                 // Log the error
//                 log::error!("Error processing Discord message: {:?}", err);

//                 // Reply with the error.
//                 if let Err(why) = msg.channel_id.say(&ctx.http, err).await {
//                     log::error!("Error sending Discord message: {:?}", why)
//                 }
//             }
//         }
//     }

//     async fn ready(&self, _: Context, ready: Ready) {
//         log::info!("Connected to Discord as {}", ready.user.name);
//     }
// }
