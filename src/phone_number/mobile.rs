use std::str::FromStr;
use strum::EnumString;

use crate::{
    impl_trait_for_string_types,
    utils::{create_fixed_map, FixedMap},
};

#[cfg(any(feature = "regex", feature = "regex_lite"))]
static MOBILE_NUMBER_REGEX: lazy_regex::Lazy<lazy_regex::Regex> =
    lazy_regex::lazy_regex!(r#"^(\+98|0|98|0098)?(9\d{2})(\d{3})(\d{4})$"#);

/// List of Iranian mobile operators.
// in future phf crate if support enums as key we must replace str with enum
pub static IRAN_MOBILE_OPERATORS: FixedMap<&str, &[&str]> = create_fixed_map! {
    "MCI" => {
        &[
            "0910", "0911", "0912", "0913", "0914", "0915", "0916", "0917", "0918", "0919",
            "0990", "0991", "0992", "0993", "0994",
        ]
    },
    "Irancell" => {
        &[
            "0930", "0933", "0935", "0936", "0937", "0938", "0939", "0901", "0902", "0903",
            "0904", "0905", "0941",
        ]
    },
    "RightTel" => {
        &["0920", "0921", "0922"]
    },
    "Taliya" => {
        &["0932"]
    },
    "MTCE" => {
    &["0931"]
    },
    "TeleKish" => {
        &["0934"]
    },
    "ApTel" => {
        &["099910", "099911", "099913"]
    },
    "Azartel" => {
        &["099914"]
    },
    "SamanTel" => {
        &["099999", "099998", "099997", "099996"]
    },
    "LotusTel" => {
    &["09990"]
    },
    "ShatelMobile" => {
        &["099810", "099811", "099812", "099814", "099815"]
    },
    "ArianTel" => {
        &["09998"]
    },
    "Anarestan" => {
        &["0994"]
    },

};

#[derive(Debug, PartialEq, Eq, Hash, EnumString)]
pub enum IranMobileOperator {
    MCI,
    MTCE,
    TeleKish,
    ApTel,
    Azartel,
    SamanTel,
    LotusTel,
    ArianTel,
    Anarestan,
    Irancell,
    RightTel,
    Taliya,
    ShatelMobile,
}

/// A trait helper to work with mobile numbers.
pub trait MobileNumber: AsRef<str> {
    /// Check if the mobile number is valid.
    #[cfg(any(feature = "regex", feature = "regex_lite"))]
    fn is_valid_mobile_number(&self) -> bool {
        MOBILE_NUMBER_REGEX.is_match(self.as_ref())
    }

    /// Check if the mobile number is valid.
    #[cfg(not(any(feature = "regex", feature = "regex_lite")))]
    fn is_valid_mobile_number(&self) -> bool {
        let text = self.as_ref();
        let skip = super::get_num_skip(text);

        if text.len() - skip != 10 {
            return false;
        }

        let mut chars = text.chars().skip(skip);

        if !chars.next().is_some_and(|c| c == '9') {
            return false;
        }

        chars.all(|c| c.is_ascii_digit())
    }

    /// Get the operator name of the mobile number.
    #[cfg(any(feature = "regex", feature = "regex_lite"))]
    fn get_operator_name_from_mobile_number(&self) -> crate::Result<Option<IranMobileOperator>> {
        let number = MOBILE_NUMBER_REGEX
            .captures(self.as_ref())
            .map(|c| format!("0{}", &self.as_ref()[c[1].len()..]))
            .ok_or("Invalid mobile number")?;

        Ok(IRAN_MOBILE_OPERATORS.iter().find_map(|(k, v)| {
            if v.iter().any(|x| x == &&number[..x.len()]) {
                Some(IranMobileOperator::from_str(k).unwrap())
            } else {
                None
            }
        }))
    }

    /// Get the operator name of the mobile number.
    #[cfg(not(any(feature = "regex", feature = "regex_lite")))]
    fn get_operator_name_from_mobile_number(&self) -> crate::Result<Option<IranMobileOperator>> {
        let text = self.as_ref();
        let skip = super::get_num_skip(text);

        if text.len() - skip != 10 {
            return Err("Invalid mobile number".into());
        }

        let number: String = "0".chars().chain(text.chars().skip(skip)).collect();

        Ok(IRAN_MOBILE_OPERATORS.iter().find_map(|(k, v)| {
            if v.iter().any(|x| x == &&number[..x.len()]) {
                Some(IranMobileOperator::from_str(k).unwrap())
            } else {
                None
            }
        }))
    }
}

impl_trait_for_string_types!(MobileNumber);
