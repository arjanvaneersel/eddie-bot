mod admins;
mod faucets;
pub mod storage;
mod user_wallets;

pub use storage::Storage;

#[cfg(test)]
mod tests {
    use super::storage::{Storage, StorageError};
    use crate::origin::Origin;

    #[test]
    fn user_wallets_flow_works() {
        let discord_user = Origin::Discord("1234".into());
        let telegram_user = Origin::Telegram("4321".into());
        let pub_addr = String::from("W1234321");

        let storage = Storage::new("/tmp/user_wallets_flow_works.edb").unwrap();

        assert!(storage
            .set_user_wallet(discord_user.clone(), pub_addr.clone())
            .is_ok());

        assert_eq!(
            storage.get_user_wallet(discord_user.clone()).unwrap(),
            pub_addr.clone()
        );

        assert!(storage
            .set_user_wallet(telegram_user.clone(), pub_addr.clone())
            .is_ok());

        assert_eq!(
            storage.get_user_wallet(telegram_user.clone()).unwrap(),
            pub_addr.clone()
        );

        assert_eq!(
            storage.get_pub_address_origins(pub_addr.clone()).unwrap(),
            vec![discord_user.clone(), telegram_user.clone()]
        );

        assert_eq!(
            storage.get_user_wallet(discord_user.clone()).unwrap(),
            pub_addr.clone()
        );

        assert!(matches!(
            storage.get_user_wallet(Origin::Discord("blah".into())),
            Err(StorageError::NotFound)
        ));

        assert!(storage
            .set_user_wallet(discord_user.clone(), String::from("W43563463"))
            .is_ok());
    }

    #[test]
    fn admin_flow_works() {
        let discord_user = Origin::Discord("1234".into());
        let telegram_user = Origin::Telegram("4321".into());

        let storage = Storage::new("/tmp/admin_flow_works.edb").unwrap();
        assert!(storage.set_admin(discord_user.clone(), false).is_ok());
        assert!(storage.is_admin(discord_user));

        assert!(!storage.is_admin(telegram_user));
        assert!(storage.has_admins());
    }
}
