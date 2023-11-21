use std::str::FromStr;
use strum::EnumString;

use crate::utils::*;

/// List of Iranian mobile operators.
// in future phf crate if support enums as key we must replace str with enum
pub static IRAN_MOBILE_OPERATORS: phf::Map<&str, &[&str]> = phf::phf_map! {
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
        &["099990", "099999", "099998", "099997", "099996"]
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
    fn is_valid_mobile_number(&self) -> bool {
        let text = self.as_ref();
        let skip = super::get_num_skip(text);

        if text.len() - skip != 10 {
            return false;
        }

        let mut chars = text.chars().skip(skip);

        chars.next().is_some_and(|c| c == '9') && chars.all(|c| c.is_ascii_digit())
    }

    /// Get the operator name of the mobile number.
    fn get_operator_name_from_mobile_number(&self) -> crate::Result<IranMobileOperator> {
        let text = self.as_ref();
        let skip = super::get_num_skip(text);

        if text.len() - skip != 10 {
            return Err("Invalid mobile number".into());
        }

        let number = format!("0{}", &text[skip..]);

        IRAN_MOBILE_OPERATORS
            .into_iter()
            .find_map(|(k, v)| {
                v.iter()
                    .any(|x| x == &&number[..x.len()])
                    .then(|| IranMobileOperator::from_str(k).unwrap())
            })
            .ok_or("Can't find the operator".into())
    }
}

impl_trait_for_string_types!(MobileNumber);
