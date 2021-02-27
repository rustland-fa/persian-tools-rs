use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PERSIAN_STR: Regex = Regex::new(r"^[\u0600-\u06FF]+$").unwrap();
    static ref HAS_PERSIAN_CHAR: Regex = Regex::new(r"[\u0600-\u06FF]").unwrap();
}

/// Set of helpers for manipulating Persian text.
trait PersianContent: AsRef<str> {
    /// Checks if a text has at least a Persian char in it.
    fn has_persian_char(&self) -> bool {
        HAS_PERSIAN_CHAR.is_match(self.as_ref())
    }

    /// Checks if a text is in Persian.
    fn is_persian_str(&self) -> bool {
        PERSIAN_STR.is_match(self.as_ref())
    }
}

impl PersianContent for String {}
impl PersianContent for str {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn has_persian_char_test() {
        assert!("ok this is text with ص".has_persian_char());
        assert!(!"ok this is text with".has_persian_char());
        assert!(!"阴阳".has_persian_char());
    }
    #[test]
    fn is_persian_str_test() {
        assert!("سلام".is_persian_str());
        assert!(!"ok this is text with".is_persian_str());
        assert!(!"阴阳".is_persian_str());
    }
}
