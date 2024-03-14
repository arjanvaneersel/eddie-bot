use crate::discord::discord::{Context, Error};
use eddie_lib::{origin::Origin, Call, Config as BotConfig, Response};
use support::traits::Dispatch;

use super::Config;

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help<T: Config + BotConfig>(
    ctx: Context<'_, T>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            // extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Get bot info
///
/// Enter `$info` to get the bot's version
#[poise::command(prefix_command, slash_command)]
pub async fn info<T: Config + BotConfig>(ctx: Context<'_, T>) -> Result<(), Error> {
    let origin = Origin::Discord(ctx.author().id.to_string());
    if let Some(Response::Info(info)) = (Call::<T>::Info).dispatch(origin)? {
        log::info!("Received bot response: {:?}", info);
        if let Err(why) = ctx.reply(info).await {
            log::error!("Couldn't send answer to Discord: {:?}", why);
        }
    }

    Ok(())
}

/// Initialize the bot
///
/// Enter `$init` to initialize the bot
#[poise::command(prefix_command, slash_command)]
pub async fn init<T: Config + BotConfig>(ctx: Context<'_, T>) -> Result<(), Error> {
    let origin = Origin::Discord(ctx.author().id.to_string());
    (Call::<T>::Init).dispatch(origin)?;
    Ok(())
}

// /// Vote for something
// ///
// /// Enter `$vote beamish` to vote for beamish
// #[poise::command(prefix_command, slash_command)]
// pub async fn vote<T: Config + BotConfig>(
//     ctx: Context<'_, T>,
//     #[description = "What beer to vote for"] choice: String,
// ) -> Result<(), Error> {
//     let choice = choice.to_lowercase();
//     let response = match choice.as_str() {
//         "heineken" => format!("We vote about beer, not cat piss!"),
//         "beamish" => format!("Now the craic is 90!"),
//         beer => format!("Your vote for {} has been registered!", beer),
//     };
//     ctx.reply(response).await?;
//     Ok(())
// }
