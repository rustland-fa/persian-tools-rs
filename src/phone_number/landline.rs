use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref LANDLINE_NUMBER_REGEX: Regex =
        Regex::new(r"(\+98|0|98|0098)?([1-9]{2})+(\d{8})$").unwrap();
}

pub fn is_valid_landline_number<T: AsRef<str>>(number: T) -> bool {
    LANDLINE_NUMBER_REGEX.is_match(number.as_ref())
}

pub fn get_prefix_landline_number<T: AsRef<str>>(number: T) -> Option<String> {
    LANDLINE_NUMBER_REGEX
        .captures(number.as_ref())
        .map(|c| format!("0{}", &c[2]))
}
