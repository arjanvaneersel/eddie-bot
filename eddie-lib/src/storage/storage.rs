use kv::Json;
use serde::{Deserialize, Serialize};

pub struct Nil;
impl kv::Value for Nil {
    fn to_raw_value(&self) -> Result<kv::Raw, kv::Error> {
        Ok(kv::Raw::from(&[0; 0]))
    }

    fn from_raw_value(_r: kv::Raw) -> Result<Self, kv::Error> {
        Ok(Nil)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaucetData {
    pub rpc_url: String,
    pub wallet_seed: String, // TODO: Implement a more secure way of storing seeds.
}

/// Holds storage functionality for the bot.
pub struct Storage<'a> {
    // db: TinyBase,
    pub user_wallets: kv::Bucket<'a, String, String>,
    pub admins: kv::Bucket<'a, String, Nil>,
    pub faucets: kv::Bucket<'a, String, Json<FaucetData>>,
}

#[derive(Debug)]
/// Enum holding all possible storage errors.
pub enum StorageError {
    /// The requested item was not found
    NotFound,

    /// The item is not unique (already exists)
    NotUnique,

    /// An invalid origin was encountered
    InvalidOrigin,

    /// An error occured when interacting with the underlying database
    Kv(kv::Error),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            StorageError::NotFound => String::from("Record not found"),
            StorageError::NotUnique => String::from("Record ID is not unique"),
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
        let faucets = store.bucket::<String, Json<FaucetData>>(Some("faucets"))?;

        Ok(Self {
            user_wallets,
            admins,
            faucets,
        })
    }
}
