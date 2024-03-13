pub mod bot;
pub mod call;
pub mod config;
pub mod functions;
pub mod origin;
pub mod storage;

pub use bot::Bot;
pub use call::{Call, Response};
pub use config::Config;

#[cfg(test)]
mod tests {
    use crate::{call::Response, origin::Origin};

    use super::*;
    use support::{param, traits::Dispatch};

    struct Test;

    param!(Name, &'static str, "Eddie");
    param!(WalletSeed, &'static str, "\\Alice");
    param!(SubstrateRPC, &'static str, "ws://localhost:9944");

    impl support::traits::Config for Test {}

    impl Config for Test {
        type Name = Name;
        type WalletSeed = WalletSeed;
        type SubstrateRPC = SubstrateRPC;
    }

    #[test]
    fn it_works() {
        let result = Call::<Test>::Version.dispatch(Origin::Telegram("1234".into()));
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Response::Version("0.1.0".to_string())),
        )
    }
}
