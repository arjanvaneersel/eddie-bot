use crate::{
    call::{Call, Response},
    config::Config,
};
use support::traits::{
    dispatch::{DispatchError, DispatchResult},
    Dispatch,
};

const MODULE: &'static str = "BOT";

pub type Result<T> = std::result::Result<T, BotError>;

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

impl<T: Config> Dispatch for Bot<T> {
    type Origin = ();
    type Call = Call;
    type Response = Option<Response>;

    fn dispatch(&self, _origin: Self::Origin, call: Self::Call) -> DispatchResult<Self::Response> {
        match call {
            Call::Version => self.do_version(),
        }
    }
}
