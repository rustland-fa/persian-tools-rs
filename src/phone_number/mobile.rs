use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use strum::EnumString;

lazy_static! {
    static ref MOBILE_NUMBER_REGEX: Regex =
        Regex::new(r"^(\+98|0|98|0098)?(9\d{2})(\d{3})(\d{4})$").unwrap();
}

/// List of Iranian mobile operators.
// in future phf crate if support enums as key we must replace str with enum
pub static IRAN_MOBILE_OPERATORS: phf::Map<&'static str, &'static [&'static str]> = phf::phf_map! {
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
            }

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
        MOBILE_NUMBER_REGEX.is_match(self.as_ref())
    }

    /// Get the operator name of the mobile number.
    fn get_operator_name_from_mobile_number(&self) -> crate::Result<Option<IranMobileOperator>> {
        let number = MOBILE_NUMBER_REGEX
            .captures(self.as_ref())
            .map(|c| format!("0{}", &self.as_ref()[c[1].len()..]))
            .ok_or("Invalid mobile number")?;

        Ok(IRAN_MOBILE_OPERATORS.into_iter().find_map(|(k, v)| {
            if v.iter().any(|x| x == &&number[..x.len()]) {
                Some(IranMobileOperator::from_str(k).unwrap())
            } else {
                None
            }
        }))
    }
}

impl MobileNumber for String {}

impl MobileNumber for str {}
