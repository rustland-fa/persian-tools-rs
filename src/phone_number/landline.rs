use std::str::FromStr;

use lazy_regex::{Lazy, Regex, lazy_regex};

use crate::{
    impl_trait_for_string_types,
    province::{IranProvince, PROVINCES},
};

static LANDLINE_NUMBER_REGEX: Lazy<Regex> = lazy_regex!(r"^(\+98|0|98|0098)?([1-9]{2})(\d{8})$");

/// A trait helper to work with landline numbers.
pub trait LandlineNumber: AsRef<str> {
    /// Check if the landline number is valid.
    fn is_valid_landline_number(&self) -> bool {
        LANDLINE_NUMBER_REGEX.is_match(self.as_ref())
    }

    /// Get three-digit prefix of a landline number.
    fn get_prefix_landline_number(&self) -> crate::Result<String> {
        LANDLINE_NUMBER_REGEX
            .captures(self.as_ref())
            .map(|c| format!("0{}", &c[2]))
            .ok_or_else(|| "Invalid landline number".into())
    }

    /// Get province of the landline number.
    fn get_province_from_landline_number(&self) -> crate::Result<Option<IranProvince>> {
        self.get_prefix_landline_number().map(|p| {
            PROVINCES.iter().find_map(|(k, v)| {
                if v.prefix_phone == p {
                    Some(IranProvince::from_str(k).unwrap())
                } else {
                    None
                }
            })
        })
    }
}

impl_trait_for_string_types!(LandlineNumber);
