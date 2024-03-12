use crate::{bot::Bot, config::Config};
use support::traits::dispatch::DispatchResult;

impl<T: Config> Bot<T> {
    pub fn do_test(&self) -> DispatchResult<()> {
        log::info!("Test worked!");
        Ok(())
    }
}
