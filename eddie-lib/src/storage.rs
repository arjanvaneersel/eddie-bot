use crate::origin::Origin;

struct Nil;
impl kv::Value for Nil {
    fn to_raw_value(&self) -> Result<kv::Raw, kv::Error> {
        Ok(kv::Raw::from(&[0; 0]))
    }

    fn from_raw_value(_r: kv::Raw) -> Result<Self, kv::Error> {
        Ok(Nil)
    }
}

/// Holds storage functionality for the bot.
pub struct Storage<'a> {
    // db: TinyBase,
    user_wallets: kv::Bucket<'a, String, String>,
    admins: kv::Bucket<'a, String, Nil>,
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

        // Initialiaze buckers.
        let user_wallets = store.bucket::<String, String>(Some("user_wallets"))?;
        let admins = store.bucket::<String, Nil>(Some("admins"))?;

        Ok(Self {
            user_wallets,
            admins,
        })
    }
}

impl<'a> Storage<'a> {
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

impl<'a> Storage<'a> {
    /// Set a user's admin status
    pub fn set_admin(&self, origin: Origin, remove: bool) -> Result<(), StorageError> {
        if !remove {
            // Store the admin
            self.admins.set(&origin.to_string(), &Nil)?;
        } else {
            self.admins.remove(&origin.to_string())?;
        }

        Ok(())
    }

    /// Get the public address from a user origin.
    pub fn is_admin(&self, origin: Origin) -> bool {
        match self.admins.get(&origin.to_string()) {
            Err(_) => false,
            Ok(v) => v.is_some(),
        }
    }

    /// Get whether there are admins appointed.
    pub fn has_admins(&self) -> bool {
        self.admins.iter().count() > 0
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
