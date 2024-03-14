use super::storage::{Nil, Storage, StorageError};
use crate::origin::Origin;

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
