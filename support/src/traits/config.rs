use std::fmt::Debug;

/// Base definition of a Config trait;
pub trait Config: 'static + Send + Sync {}

/// A type that can be used in runtime structures.
pub trait Member: Send + Sync + Sized + Debug + Eq + PartialEq + Clone + 'static {}
impl<T: Send + Sync + Sized + Debug + Eq + PartialEq + Clone + 'static> Member for T {}
