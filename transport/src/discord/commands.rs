use crate::discord::discord::{Context, Error};
use eddie_lib::{Call, Config as BotConfig, Response};
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
                extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
                ..Default::default()
            },
        )
        .await?;
    Ok(())
}

/// Get the bot version
///
/// Enter `$version` to get the bot's version
#[poise::command(prefix_command, slash_command)]
pub async fn version<T: Config + BotConfig>(ctx: Context<'_, T>) -> Result<(), Error> {
    let response = (Call::<T>::Version).dispatch(());
    log::info!("Received bot response: {:?}", response);
    match response {
        Ok(Some(Response::Version(version))) => {
            if let Err(why) = ctx.reply(version).await {
                log::error!("Couldn't send answer to Discord: {:?}", why);
            }
        }
        Ok(None) => {}
        Err(err) => Err(err)?,
    }

    Ok(())
}

/// Vote for something
///
/// Enter `$vote beamish` to vote for beamish
#[poise::command(prefix_command, slash_command)]
pub async fn vote<T: Config + BotConfig>(
    ctx: Context<'_, T>,
    #[description = "What beer to vote for"] choice: String,
) -> Result<(), Error> {
    let choice = choice.to_lowercase();
    let response = match choice.as_str() {
        "heineken" => format!("We vote about beer, not cat piss!"),
        "beamish" => format!("Now the craic is 90!"),
        beer => format!("Your vote for {} has been registered!", beer),
    };
    ctx.reply(response).await?;
    Ok(())
}
