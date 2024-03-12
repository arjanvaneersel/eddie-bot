use crate::{bot::Bot, call::Response, config::Config};

/// Type to make function definitions a bit cleaner.
type DispatchResult<T> = support::traits::dispatch::DispatchResult<Option<T>>;

/// The version as defined in Cargo.toml.
const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

impl<T: Config> Bot<T> {
    pub fn do_version(&self) -> DispatchResult<Response> {
        Ok(Some(Response::Version(format!(
            "{}",
            VERSION.unwrap_or("unknown")
        ))))
    }
}
