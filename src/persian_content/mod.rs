use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PERSIAN_STR: Regex = Regex::new(r"^[\u0600-\u06FF]+$").unwrap();
    static ref HAS_PERSIAN_CHAR: Regex = Regex::new(r"[\u0600-\u06FF]").unwrap();
}

pub fn has_persian_char<T: AsRef<str>>(s: T) -> bool {
    HAS_PERSIAN_CHAR.is_match(s.as_ref())
}

pub fn is_persian_str<T: AsRef<str>>(s: T) -> bool {
    PERSIAN_STR.is_match(s.as_ref())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn has_persian_char_test() {
        assert!(has_persian_char("ok this is text with ص"));
        assert_eq!(has_persian_char("ok this is text with"), false);
    }
    #[test]
    fn is_persian_str_test() {
        assert!(is_persian_str("سلام"));
        assert_eq!(is_persian_str("ok this is text with"), false);
    }
}
