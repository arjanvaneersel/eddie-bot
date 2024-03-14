use super::storage::{Storage, StorageError};
use crate::origin::Origin;

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
