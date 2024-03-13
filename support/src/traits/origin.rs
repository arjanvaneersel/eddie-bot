/// Trait for dealing with origin objects.
pub trait Origin {
    fn network(&self) -> String;
    fn user_id(&self) -> String;
}
