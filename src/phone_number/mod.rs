use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PHONE_NUMBER_REGEX: Regex =
        Regex::new(r"(?:[+|0{2}]?98)?(?:0)?(\d{3})+(\d{3})+(\d{4})$").unwrap();
}

pub fn is_valid_phone_number<T: AsRef<str>>(number: T) -> bool {
    PHONE_NUMBER_REGEX.is_match(number.as_ref())
}

pub fn get_prefix_phone_number<T: AsRef<str>>(number: T) -> Option<String> {
    let capture = PHONE_NUMBER_REGEX.captures(number.as_ref());
    if let Some(c) = capture {
        return Some(c[0].to_string());
    }
    None
}
