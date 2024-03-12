use support::traits::Get;

/// Config trait for bot logic.
pub trait Config {
    /// Type holding the bot's name.
    type Name: Get<&'static str>;

    /// Type holding the bot's wallet seed.
    type WalletSeed: Get<&'static str>;

    /// Type holding the rpc address.
    type SubstrateRPC: Get<&'static str>;
}
