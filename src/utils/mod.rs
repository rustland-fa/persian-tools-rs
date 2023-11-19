pub use fixed_map::FixedMap;

mod fixed_map;

#[macro_export]
macro_rules! impl_trait_for_string_types {
    ($name_trait:ident) => {
        impl $name_trait for String {}
        impl $name_trait for str {}
        // maybe other string type
    };
}

macro_rules! create_fixed_map {
    ($($key:literal => $value:expr,)*) => {
        FixedMap(&[$(($key, $value)),*])
    };
}

pub(crate) use create_fixed_map;
