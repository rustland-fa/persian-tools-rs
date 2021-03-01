use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MOBILE_NUMBER_REGEX: Regex =
        Regex::new(r"^(\+98|0|98|0098)?(9\d{2})(\d{3})(\d{4})$").unwrap();
}
/// iran mobile operator
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

#[derive(Debug, PartialEq, Eq)]
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

// TODO maybe use FromStr is better
impl From<&str> for IranMobileOperator {
    fn from(t: &str) -> Self {
        match t.to_lowercase().as_str() {
            "mci" => IranMobileOperator::MCI,
            "mtce" => IranMobileOperator::MTCE,
            "telekish" => IranMobileOperator::TeleKish,
            "aptel" => IranMobileOperator::ApTel,
            "taliya" => IranMobileOperator::Taliya,
            "azartel" => IranMobileOperator::Azartel,
            "samantel" => IranMobileOperator::SamanTel,
            "lotustel" => IranMobileOperator::LotusTel,
            "ariantel" => IranMobileOperator::ArianTel,
            "anarestan" => IranMobileOperator::Anarestan,
            "irancell" => IranMobileOperator::Irancell,
            "rightel" => IranMobileOperator::RightTel,
            "shatelmobile" => IranMobileOperator::ShatelMobile,
            _ => panic!("invalid input"),
        }
    }
}

/// trait helper for mobile number
pub trait MobileNumber: AsRef<str> {
    /// check mobile number is valid
    fn is_valid_mobile_number(&self) -> bool {
        MOBILE_NUMBER_REGEX.is_match(self.as_ref())
    }
    /// get operator name mobile number
    fn get_operator_name_from_mobile_number(&self) -> crate::Result<Option<IranMobileOperator>> {
        let number = MOBILE_NUMBER_REGEX
            .captures(self.as_ref())
            .map(|c| format!("0{}", &self.as_ref()[c[1].len()..]))
            .ok_or_else(|| "invalid mobile number")?;
        Ok(IRAN_MOBILE_OPERATORS.into_iter().find_map(|(k, v)| {
            if v.iter().any(|x| x == &&number[..x.len()]) {
                Some((*k).into())
            } else {
                None
            }
        }))
    }
}

impl MobileNumber for String {}

impl MobileNumber for str {}
