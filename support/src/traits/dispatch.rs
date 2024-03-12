#[derive(Debug, Clone)]
/// Collection of all possible dispatch errors.
pub enum DispatchError {
    Module(String, String),
    Other(String),
}

impl std::fmt::Display for DispatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            DispatchError::Module(modl, err) => format!("{} error in module {}", err, modl),
            DispatchError::Other(err) => err.clone(),
        };

        write!(f, "{}", err)
    }
}

impl std::error::Error for DispatchError {}

/// Type that models dispatch results.
pub type DispatchResult<T> = std::result::Result<T, DispatchError>;

/// Trait for dispatching calls.
pub trait Dispatch {
    type Origin;
    type Response;

    fn dispatch(&self, origin: Self::Origin) -> DispatchResult<Self::Response>;
}
