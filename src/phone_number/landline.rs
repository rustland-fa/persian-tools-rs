use lazy_static::lazy_static;
use regex::Regex;

use crate::province::{IranProvince, PROVINCES};

lazy_static! {
    static ref LANDLINE_NUMBER_REGEX: Regex =
        Regex::new(r"^(\+98|0|98|0098)?([1-9]{2})(\d{8})$").unwrap();
}
/// trait helper for validate landnumber and more ...
pub trait LandlineNumber: AsRef<str> {
    /// check landline number is valid number
    fn is_valid_landline_number(&self) -> bool {
        LANDLINE_NUMBER_REGEX.is_match(self.as_ref())
    }
    /// get three-digit prefix landline number
    fn get_prefix_landline_number(&self) -> crate::Result<String> {
        LANDLINE_NUMBER_REGEX
            .captures(&self.as_ref())
            .map(|c| format!("0{}", &c[2]))
            .ok_or_else(|| "invalid landline number".into())
    }
    // get province from landline number
    fn get_province_from_landline_number(&self) -> crate::Result<Option<IranProvince>> {
        self.get_prefix_landline_number()
            .map(|p| {
                PROVINCES.into_iter().find_map(
                    |(k, v)| {
                        if v.prefix_phone == p {
                            Some(k)
                        } else {
                            None
                        }
                    },
                )
            })
            .map(|p| p.map(|&j| j.into()))
    }
}

impl LandlineNumber for String {}

impl LandlineNumber for str {}
