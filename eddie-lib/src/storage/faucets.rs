use kv::Json;

use super::storage::{FaucetData, Storage, StorageError};
use crate::origin::Origin;

impl<'a> Storage<'a> {
    pub fn register_faucet_channel(&self, channel: &Origin) -> Result<(), StorageError> {
        // Return an error is the channel is already registered.
        if self.faucets.get(&channel.to_string())?.is_some() {
            return Err(StorageError::NotUnique);
        }

        // Store the channel with empty values.
        let data = Json(FaucetData {
            rpc_url: String::new(),
            wallet_seed: String::new(),
        });
        self.faucets.set(&channel.to_string(), &data)?;

        Ok(())
    }

    pub fn activate_faucet_channel(
        &self,
        channel: &Origin,
        rpc_url: &String,
        wallet_seed: &String,
    ) -> Result<(), StorageError> {
        // Return an error is the channel is not registered.
        if self.faucets.get(&channel.to_string())?.is_none() {
            return Err(StorageError::NotUnique);
        }

        // Store the channel with empty values.
        let data = Json(FaucetData {
            rpc_url: rpc_url.clone(),
            wallet_seed: wallet_seed.clone(),
        });
        self.faucets.set(&channel.to_string(), &data)?;

        Ok(())
    }
}
