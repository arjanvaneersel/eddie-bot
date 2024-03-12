use crate::{call::Call, config::Config};
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
        DispatchError::Module(String::From(MODULE), format!("{}", value))
    }
}

pub struct Bot<T: Config>(std::marker::PhantomData<T>);

impl<T: Config> Dispatch for Bot<T> {
    type Origin = ();
    type Call = Call;
    type Answer = ();

    fn dispatch(&self, origin: Self::Origin, call: Self::Call) -> DispatchResult<Self::Answer> {
        todo!()
    }
}
