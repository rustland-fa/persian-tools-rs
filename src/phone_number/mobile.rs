use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MOBILE_NUMBER_REGEX: Regex =
        Regex::new(r"(?:[+|0{2}]?98)?(?:0)?(\d{3})+(\d{3})+(\d{4})$").unwrap();
}

pub enum IranMobileOperator {
    MCI,
    IranCell,
    RightTel,
    Taliya,
    ShatelMobile,
}

impl IranMobileOperator {
    // TODO
    pub fn values(&self) -> Vec<&str> {
        use IranMobileOperator::*;
        match self {
            MCI => {
                vec![]
            }
            IranCell => {
                vec![]
            }
            RightTel => {
                vec![]
            }
            Taliya => {
                vec![]
            }
            ShatelMobile => {
                vec![]
            }
        }
    }
}

pub fn is_valid_mobile_number<T: AsRef<str>>(number: T) -> bool {
    MOBILE_NUMBER_REGEX.is_match(number.as_ref())
}

pub fn get_prefix_mobile_number<T: AsRef<str>>(number: T) -> Option<String> {
    MOBILE_NUMBER_REGEX
        .captures(number.as_ref())
        .map(|c| c[1].to_string())
}
