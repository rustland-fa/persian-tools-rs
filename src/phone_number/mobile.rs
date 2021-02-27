use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MOBILE_NUMBER_REGEX: Regex =
        Regex::new(r"(\+98|0|98|0098)?(9\d{2})+(\d{3})+(\d{4})$").unwrap();
}

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

impl From<&str> for IranMobileOperator {
    fn from(t: &str) -> Self {
        match t.to_lowercase().as_str() {
            "mci" => IranMobileOperator::MCI,
            "mtce" => IranMobileOperator::MTCE,
            "telekish" => IranMobileOperator::TeleKish,
            "aptel" => IranMobileOperator::ApTel,
            "taliya" => IranMobileOperator::Taliya,
            _ => unimplemented!("TODO ..."),
        }
    }
}

pub fn is_valid_mobile_number<T: AsRef<str>>(number: T) -> bool {
    MOBILE_NUMBER_REGEX.is_match(number.as_ref())
}

#[deprecated(note = "please dont use this")]
pub fn get_prefix_mobile_number<T: AsRef<str>>(number: T) -> Option<String> {
    MOBILE_NUMBER_REGEX
        .captures(number.as_ref())
        .map(|c| format!("0{}", &c[2]))
}

// TODO FIXME validate number and replace prefix number +98 or 98 to 0
pub fn get_operator_name_from_mobile_number<T: AsRef<str>>(
    number: T,
) -> Option<IranMobileOperator> {
    let number = number.as_ref().to_string();
    IRAN_MOBILE_OPERATORS.into_iter().find_map(|(k, v)| {
        if v.iter().any(|x| x == &&number[..x.len()]) {
            Some((*k).into())
        } else {
            None
        }
    })
}
