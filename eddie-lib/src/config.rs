use support::traits::Get;

/// Config trait for bot logic.
pub trait Config {
    type Name: Get<&'static str>;
}
