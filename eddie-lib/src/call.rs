use support::traits::{
    dispatch::{DispatchError, DispatchResult},
    Dispatch,
};

use crate::{origin::Origin, Bot, Config};

#[derive(Clone, Debug, PartialEq)]
/// Collection of all possible calls to the bot.
pub enum Call<T: Config> {
    #[deprecated(since = "0.1.0", note = "use Info instead")]
    Version,
    Info,
    Init,
    _Unreachable(std::marker::PhantomData<T>),
}

impl<T: Config> Dispatch for Call<T> {
    type Origin = Origin;
    type Response = Option<Response>;

    fn dispatch(&self, origin: Self::Origin) -> DispatchResult<Self::Response> {
        match self {
            Call::Info => Bot::<T>::info(),
            Call::Init => Bot::<T>::init(origin),
            _ => Err(DispatchError::Other(String::from("Unsupported call"))),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Collection of all possible responses from the bot.
pub enum Response {
    Version(String),
    Info(String),
}
