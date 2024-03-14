use crate::discord::discord::{Context, Error};
use eddie_lib::{origin::Origin, Call, Config as BotConfig, Response};
use poise::serenity_prelude::ChannelId;
use support::traits::Dispatch;

use super::Config;

async fn parse_call<T: Config + BotConfig>(
    ctx: Context<'_, T>,
    response: Option<Response>,
) -> Result<(), Error> {
    match response {
        None => {}
        Some(Response::Say(msg)) => {
            if let Err(err) = ctx.say(msg).await {
                log::error!("Couldn't send message to Discord: {:?}", err);
            }
        }
        Some(Response::SayChan(reply_channel, msg)) => {
            if let Ok(channel_id) = reply_channel.inner().parse::<u64>() {
                let channel = ChannelId::from(channel_id);
                if let Err(err) = channel.say(ctx.http(), msg).await {
                    log::error!("Couldn't send message to Discord: {:?}", err);
                }
            } else {
                log::error!("Invalid Discord channel ID: {}", reply_channel.inner())
            }
        }
        Some(Response::Reply(msg)) => {
            if let Err(err) = ctx.reply(msg).await {
                log::error!("Couldn't send reply to Discord: {:?}", err);
            }
        }
        Some(Response::ReplyDirect(msg)) => {
            let dm_channel = ctx.author().id.create_dm_channel(ctx.http()).await?;
            if let Err(why) = dm_channel.say(ctx.http(), msg).await {
                log::error!("Couldn't send DM to Discord: {:?}", why);
            }
        }
    }
    Ok(())
}

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
    parse_call(ctx, Call::<T>::Info.dispatch(origin)?).await
    // if let Some(Response::Reply(info)) = (Call::<T>::Info).dispatch(origin)? {
    //     log::info!("Received bot response: {:?}", info);
    //     if let Err(why) = ctx.reply(info).await {
    //         log::error!("Couldn't send answer to Discord: {:?}", why);
    //     }
    // }

    // Ok(())
}

/// Initialize the bot
///
/// Enter `$init` to initialize the bot
#[poise::command(prefix_command, slash_command)]
pub async fn init<T: Config + BotConfig>(ctx: Context<'_, T>) -> Result<(), Error> {
    let origin = Origin::Discord(ctx.author().id.to_string());
    parse_call(ctx, (Call::<T>::Init).dispatch(origin)?).await
    // (Call::<T>::Init).dispatch(origin)?;
    // Ok(())
}

/// Initialize the bot
///
/// Enter `$init` to initialize the bot
#[poise::command(prefix_command, slash_command)]
pub async fn set_admin<T: Config + BotConfig>(
    ctx: Context<'_, T>,
    admin: String,
    remove: bool,
) -> Result<(), Error> {
    let origin = Origin::Discord(ctx.author().id.to_string());
    let admin_origin = Origin::try_from(admin)?;
    parse_call(
        ctx,
        (Call::<T>::SetAdmin(admin_origin, remove)).dispatch(origin)?,
    )
    .await
}

/// Register a channel as a faucet channel
///
/// Enter `$register_faucet` to register a channel as a faucet
#[poise::command(prefix_command, slash_command)]
pub async fn register_faucet<T: Config + BotConfig>(ctx: Context<'_, T>) -> Result<(), Error> {
    let who = Origin::Discord(ctx.author().id.to_string());
    let channel = Origin::Discord(ctx.channel_id().to_string());
    parse_call(
        ctx,
        Call::<T>::RegisterFaucetChannel(channel).dispatch(who)?,
    )
    .await
    // if let Some(Response::ReplyDirect(msg)) =
    //     (Call::<T>::RegisterFaucetChannel(channel)).dispatch(who)?
    // {
    //     log::info!("Received bot response: {:?}", msg);
    //     let dm_channel = ctx.author().id.create_dm_channel(ctx.http()).await?;
    //     if let Err(why) = dm_channel.say(ctx.http(), msg).await {
    //         log::error!("Couldn't send answer to Discord: {:?}", why);
    //     }
    // }
    // Ok(())
}

/// Activate a faucet channel
///
/// Enter `$register_faucet` to register a channel as a faucet
#[poise::command(prefix_command, slash_command)]
pub async fn activate_faucet<T: Config + BotConfig>(
    ctx: Context<'_, T>,
    channel_id: String,
    rpc_url: String,
    wallet_seed: String,
) -> Result<(), Error> {
    if let Err(why) = ctx.reply("Activating now...").await {
        log::error!("Couldn't send answer to Discord: {:?}", why);
    }

    let who = Origin::Discord(ctx.author().id.to_string());
    let channel = Origin::Discord(channel_id.clone());
    parse_call(
        ctx,
        Call::<T>::ActivateFaucetChannel {
            channel,
            rpc_url,
            wallet_seed,
        }
        .dispatch(who)?,
    )
    .await
    // if let Some(Response::SayChan(reply_channel, msg)) = (Call::<T>::ActivateFaucetChannel {
    //     channel,
    //     rpc_url,
    //     wallet_seed,
    // })
    // .dispatch(who)?
    // {
    //     log::info!("Received bot response: {:?}", msg);
    //     if let Ok(channel_id) = reply_channel.inner().parse::<u64>() {
    //         let channel = ChannelId::from(channel_id);
    //         if let Err(why) = channel.say(ctx.http(), msg).await {
    //             log::error!("Couldn't send answer to Discord: {:?}", why);
    //         }
    //     } else {
    //         log::error!("Invalid Discord channel ID: {}", channel_id)
    //     }
    // }
    // Ok(())
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
