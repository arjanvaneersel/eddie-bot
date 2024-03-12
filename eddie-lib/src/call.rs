use support::traits::{
    dispatch::{DispatchError, DispatchResult},
    Dispatch,
};

use crate::{Bot, Config};

// #[derive(Clone, Debug, PartialEq)]
// Collection of all possible origins.
// pub enum Origin {
//     Telegram,
//     Discord,
// }

#[derive(Clone, Debug, PartialEq)]
/// Collection of all possible calls to the bot.
pub enum Call<T: Config> {
    Version,
    _Unreachable(std::marker::PhantomData<T>),
}

impl<T: Config> Dispatch for Call<T> {
    type Origin = ();
    type Response = Option<Response>;

    fn dispatch(&self, _origin: Self::Origin) -> DispatchResult<Self::Response> {
        match self {
            Call::Version => Bot::<T>::version(),
            _ => Err(DispatchError::Other(String::from("Unsupported call"))),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Collection of all possible responses from the bot.
pub enum Response {
    Version(String),
}
