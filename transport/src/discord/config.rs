use eddie_lib::Config as BotConfig;
use support::traits::{Config as BaseConfig, Get};

/// Config trait for Discord transport.
pub trait Config: BaseConfig + BotConfig {
    // /// Type for calls to the bot.
    // type Call: Member;

    // /// Type for the origin that the bot will use.
    // type Origin: Member;

    // /// Type for bot responses;
    // type Response: Member;

    /// Type containing the bot config so that calls can be dispatched.
    type Bot: BotConfig;

    /// Type holding the discord token.
    type Token: Get<String>;
}
