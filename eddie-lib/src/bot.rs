use crate::config::Config;
use support::traits::dispatch::DispatchError;

pub const MODULE: &'static str = "BOT";

#[derive(Clone, Debug)]
/// All potential bot errors.
pub enum BotError {
    /// An unknown error has occured.
    UnknownError,
}

impl std::fmt::Display for BotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            Self::UnknownError => "Unknown error",
        };

        write!(f, "{}", err)
    }
}

impl std::error::Error for BotError {}

impl From<BotError> for DispatchError {
    fn from(value: BotError) -> Self {
        DispatchError::Module(String::from(MODULE), format!("{}", value))
    }
}

/// Default bot
// TODO: Consider whether the bot and bot logic should be here or in the bin crate.
pub struct Bot<T: Config>(std::marker::PhantomData<T>);

impl<T: Config> Bot<T> {
    pub fn new() -> Bot<T> {
        Bot(std::marker::PhantomData)
    }
}
