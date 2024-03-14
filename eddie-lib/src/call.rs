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
    SetAdmin(Origin, bool),
    RegisterFaucetChannel(Origin),
    ActivateFaucetChannel {
        channel: Origin,
        rpc_url: String,
        wallet_seed: String,
    },
    _Unreachable(std::marker::PhantomData<T>),
}

impl<T: Config> Dispatch for Call<T> {
    type Origin = Origin;
    type Response = Option<Response>;

    fn dispatch(&self, origin: Self::Origin) -> DispatchResult<Self::Response> {
        match self {
            Call::Info => Bot::<T>::info(),
            Call::Init => Bot::<T>::init(origin),
            Call::SetAdmin(admin, remove) => Bot::<T>::set_admin(origin, admin, remove.to_owned()),
            Call::RegisterFaucetChannel(channel) => {
                Bot::<T>::register_faucet_channel(origin, channel)
            }
            Call::ActivateFaucetChannel {
                channel,
                rpc_url,
                wallet_seed,
            } => Bot::<T>::activate_faucet_channel(origin, &channel, rpc_url, wallet_seed),
            _ => Err(DispatchError::Other(String::from("Unsupported call"))),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Collection of all possible responses from the bot.
pub enum Response {
    Reply(String),
    ReplyDirect(String),
    Say(String),
    SayChan(Origin, String),
}
