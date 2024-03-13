use crate::origin::Origin;

/// Holds storage functionality for the bot.
pub struct Storage<'a> {
    // db: TinyBase,
    user_wallets: kv::Bucket<'a, String, String>,
}

#[derive(Debug)]
/// Enum holding all possible storage errors.
pub enum StorageError {
    /// The requested item was not found
    NotFound,

    /// An invalid origin was encountered
    InvalidOrigin,

    /// An error occured when interacting with the underlying database
    Kv(kv::Error),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            StorageError::NotFound => String::from("Record not found"),
            StorageError::InvalidOrigin => String::from("Invalid origin"),
            StorageError::Kv(err) => format!("{}", err),
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for StorageError {}

impl From<kv::Error> for StorageError {
    fn from(value: kv::Error) -> Self {
        Self::Kv(value)
    }
}

impl<'a> Storage<'a> {
    /// Instantiate a new storage.
    pub fn new(db: &str) -> Result<Storage<'a>, StorageError> {
        // Initialize database.
        let cfg = kv::Config::new(db);
        let store = kv::Store::new(cfg)?;

        // Initialiaze tables.
        let user_wallets = store.bucket::<String, String>(Some("user_wallets"))?;

        Ok(Self { user_wallets })
    }

    /// Set a user wallet value.
    ///
    /// Overwrites existing values.
    pub fn set_user_wallet(&self, origin: Origin, pub_addr: String) -> Result<(), StorageError> {
        // TODO: Check if the address is valid.

        // Store the wallet
        self.user_wallets.set(&origin.to_string(), &pub_addr)?;
        Ok(())
    }

    /// Get the public address from a user origin.
    pub fn get_user_wallet(&self, origin: Origin) -> Result<String, StorageError> {
        Ok(self
            .user_wallets
            .get(&origin.to_string())?
            .ok_or(StorageError::NotFound)?)
    }

    /// Get all origins using the same public address.
    ///
    /// This can happen when users use both Discord and Telegram.
    pub fn get_pub_address_origins(&self, pub_addr: String) -> Result<Vec<Origin>, StorageError> {
        let mut origins: Vec<Origin> = Vec::new();
        for item in self.user_wallets.iter() {
            let item = item?;
            if item.value::<String>()? == pub_addr {
                let key = item.key::<String>()?;
                origins.push(Origin::try_from(key).map_err(|_| StorageError::InvalidOrigin)?)
            }
        }
        Ok(origins)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_wallets_flow_works() {
        let discord_user = Origin::Discord("1234".into());
        let telegram_user = Origin::Telegram("4321".into());
        let pub_addr = String::from("W1234321");

        let storage = Storage::new("/tmp/test.edb").unwrap();

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
}
