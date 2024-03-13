use eddie_lib::Config as BotConfig;
use support::traits::{Config as BaseConfig, Get};

/// Config trait for Discord transport.
pub trait Config: BaseConfig + BotConfig {
    /// Type containing the bot config so that calls can be dispatched.
    type Bot: BotConfig;

    /// Type holding the telegram token.
    type Token: Get<String>;
}
