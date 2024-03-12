/// A trait for querying a single value from a type defined in the trait.
///
/// It is not required that the value is constant.
pub trait TypedGet {
    /// The type which is returned.
    type Type;
    /// Return the current value.
    fn get() -> Self::Type;
}

/// A trait for querying a single value from a type.
///
/// It is not required that the value is constant.
pub trait Get<T> {
    fn get() -> T;
}

/// Implement Get by returning Default for any type that implements Default.
impl<T: Default> Get<T> for () {
    fn get() -> T {
        T::default()
    }
}

pub struct GetDefault;
impl<T: Default> Get<T> for GetDefault {
    fn get() -> T {
        T::default()
    }
}

/// A macro for easy const get implementations.
macro_rules! impl_const_get {
    ($name:ident, $t:ty) => {
        /// Const getter for a basic type.
        #[derive(Default, Clone)]
        pub struct $name<const T: $t>;

        impl<const T: $t> core::fmt::Debug for $name<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.write_str(&format!("{}<{}>", stringify!($name), T))
            }
        }

        impl<const T: $t> Get<$t> for $name<T> {
            fn get() -> $t {
                T
            }
        }
        impl<const T: $t> Get<Option<$t>> for $name<T> {
            fn get() -> Option<$t> {
                Some(T)
            }
        }
        impl<const T: $t> TypedGet for $name<T> {
            type Type = $t;
            fn get() -> $t {
                T
            }
        }
    };
}

// Supported consts.
impl_const_get!(ConstBool, bool);
impl_const_get!(ConstU8, u8);
impl_const_get!(ConstU16, u16);
impl_const_get!(ConstU32, u32);
impl_const_get!(ConstU64, u64);
impl_const_get!(ConstU128, u128);
impl_const_get!(ConstI8, i8);
impl_const_get!(ConstI16, i16);
impl_const_get!(ConstI32, i32);
impl_const_get!(ConstI64, i64);
impl_const_get!(ConstI128, i128);

#[macro_export]
/// A macro to define parameters that implement the Get trait.
macro_rules! param {
    ($name:ident, $t:ty, $v:expr) => {
        /// Const getter for a basic type.
        #[derive(Default, Clone)]
        pub struct $name;

        impl $name {
            pub const fn get() -> $t {
                $v
            }
        }
        impl<I: From<$t>> $crate::support::traits::get::Get<I> for $name {
            fn get() -> I {
                I::from(Self::get())
            }
        }
        impl core::fmt::Debug for $name {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.write_str(&format!("{}<{}>", stringify!($name), stringify!($t)))
            }
        }
        impl $crate::support::traits::get::TypedGet for $name {
            type Type = $t;
            fn get() -> $t {
                $v
            }
        }
    };
}
