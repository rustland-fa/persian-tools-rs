use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref PERSIAN_STR: Regex = Regex::new(r"^[\u0600-\u06FF]+$").unwrap();
}

pub fn has_persian_char<T: AsRef<str>>(s: T) -> bool {
    s.as_ref()
        .chars()
        .into_iter()
        .any(|c| PERSIAN_STR.is_match(&c.to_string()))
}

pub fn is_persian_str<T: AsRef<str>>(s: T) -> bool {
    PERSIAN_STR.is_match(s.as_ref())
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    pub fn has_persian_char_test() {
        assert!(has_persian_char("ok this is text with ص"));
        assert_eq!(has_persian_char("ok this is text with"), false);
    }
    #[test]
    pub fn is_persian_str_test() {
        assert!(is_persian_str("سلام"));
        assert_eq!(is_persian_str("ok this is text with"), false);
    }
}
