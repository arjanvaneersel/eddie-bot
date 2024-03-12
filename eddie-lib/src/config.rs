use support::traits::{Config as BaseConfig, Get};

/// Config trait for bot logic.
pub trait Config: BaseConfig {
    /// Type holding the bot's name.
    type Name: Get<String>;

    /// Type holding the bot's wallet seed.
    type WalletSeed: Get<String>;

    /// Type holding the rpc address.
    type SubstrateRPC: Get<String>;
}
