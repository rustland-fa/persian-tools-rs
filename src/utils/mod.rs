macro_rules! impl_trait_for_string_types {
    ($name_trait:ident) => {
        impl $name_trait for String {}
        impl $name_trait for str {}
        // maybe other string type
    };
}

pub(crate) use impl_trait_for_string_types;
