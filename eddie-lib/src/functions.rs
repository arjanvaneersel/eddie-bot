use crate::{bot::Bot, call::Response, config::Config, origin::Origin, storage::Storage};
use support::traits::{dispatch::DispatchError, Get};

/// Type to make function definitions a bit cleaner.
type DispatchResult<T> = support::traits::dispatch::DispatchResult<Option<T>>;

/// The version as defined in Cargo.toml.
const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

impl<T: Config> Bot<T> {
    #[deprecated(since = "0.1.0", note = "use info() instead")]
    pub fn version() -> DispatchResult<Response> {
        Ok(Some(Response::Version(format!(
            "{}",
            VERSION.unwrap_or("unknown")
        ))))
    }

    pub fn info() -> DispatchResult<Response> {
        Ok(Some(Response::Info(format!(
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
}
