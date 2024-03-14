use crate::{bot::Bot, call::Response, config::Config, origin::Origin, storage::Storage};
use support::traits::{dispatch::DispatchError, Get};

/// Type to make function definitions a bit cleaner.
type DispatchResult<T> = support::traits::dispatch::DispatchResult<Option<T>>;

/// The version as defined in Cargo.toml.
const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

impl<T: Config> Bot<T> {
    #[deprecated(since = "0.1.0", note = "use info() instead")]
    pub fn version() -> DispatchResult<Response> {
        Ok(Some(Response::Reply(format!(
            "{}",
            VERSION.unwrap_or("unknown")
        ))))
    }

    pub fn info() -> DispatchResult<Response> {
        Ok(Some(Response::Reply(format!(
            "Eddie version {}\nCopyright (c) 2024, Arjan van Eersel\n\nMore information: https://github.com/arjanvaneersel/eddie-bot",
            VERSION.unwrap_or("unknown")
        ))))
    }

    pub fn init(who: Origin) -> DispatchResult<Response> {
        // Get the storage.
        let storage = Storage::new(&T::DBPath::get().to_owned())
            .map_err(|err| DispatchError::Other(err.to_string()))?;

        // If there are already admins then the bot was already initialized,
        // thus we return an error.
        if storage.has_admins() {
            return Err(DispatchError::Module(
                crate::MODULE.into(),
                "Already initialized".into(),
            ));
        }

        storage
            .set_admin(who, false)
            .map_err(|err| DispatchError::Other(err.to_string()))?;

        Ok(None)
    }

    pub fn register_faucet_channel(who: Origin, channel: &Origin) -> DispatchResult<Response> {
        // Get the storage.
        let storage = Storage::new(&T::DBPath::get().to_owned())
            .map_err(|err| DispatchError::Other(err.to_string()))?;

        // Reject if the caller isn't an admin.
        if !storage.is_admin(who) {
            return Err(DispatchError::Module(
                crate::MODULE.into(),
                "Only an admin can perform this action".into(),
            ));
        }

        // Register the channel as a faucet in the storage.
        storage
            .register_faucet_channel(channel)
            .map_err(|err| DispatchError::Other(err.to_string()))?;

        Ok(Some(Response::ReplyDirect(format!(
            "The channel has been registered as a faucet.\n\nNow please activate the faucet by replying here with the following command:\n/activate_faucet {} <rpc url> <wallet seed>",
            channel.inner(),
        ))))
    }

    pub fn activate_faucet_channel(
        who: Origin,
        channel: &Origin,
        rpc_url: &String,
        wallet_seed: &String,
    ) -> DispatchResult<Response> {
        // Get the storage.
        let storage = Storage::new(&T::DBPath::get().to_owned())
            .map_err(|err| DispatchError::Other(err.to_string()))?;

        // Reject if the caller isn't an admin.
        if !storage.is_admin(who) {
            return Err(DispatchError::Module(
                crate::MODULE.into(),
                "Only an admin can perform this action".into(),
            ));
        }

        // Register the channel as a faucet in the storage.
        storage
            .activate_faucet_channel(channel, rpc_url, wallet_seed)
            .map_err(|err| DispatchError::Other(err.to_string()))?;

        Ok(Some(Response::SayChan(
            channel.clone(),
            "Faucet has been activated for this channel!".into(),
        )))
    }

    pub fn set_admin(who: Origin, admin: &Origin, remove: bool) -> DispatchResult<Response> {
        // Get the storage.
        let storage = Storage::new(&T::DBPath::get().to_owned())
            .map_err(|err| DispatchError::Other(err.to_string()))?;

        // Reject if the caller isn't an admin.
        if !storage.is_admin(who) {
            return Err(DispatchError::Module(
                crate::MODULE.into(),
                "Only an admin can perform this action".into(),
            ));
        }

        // Register admin status
        storage
            .set_admin(admin.clone(), remove)
            .map_err(|err| DispatchError::Other(err.to_string()))?;

        let reply = match remove {
            false => "The user is now an admin.",
            true => "The user is no longer an admin.",
        };
        Ok(Some(Response::Reply(reply.into())))
    }
}
