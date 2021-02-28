use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PERSIAN_STR: Regex = Regex::new(r"^[\u0600-\u06FF]|[[:punc:]]+$").unwrap();
    static ref HAS_PERSIAN_CHAR: Regex = Regex::new(r"[\u0600-\u06FF]").unwrap();
}

/// Set of helpers for manipulating Persian text.
pub trait PersianContent: AsRef<str> {
    /// Checks if a text has at least a Persian char in it.
    fn has_persian_char(&self) -> bool {
        HAS_PERSIAN_CHAR.is_match(self.as_ref())
    }

    /// Checks if a text is in Persian.
    fn is_persian_str(&self) -> bool {
        // First remove the non-alphabetic chars
        let string = self
            .as_ref()
            .chars()
            .into_iter()
            .filter(|c| c.is_alphabetic())
            .collect::<String>();
        PERSIAN_STR.is_match(&string)
    }

    /// Calculates how much of the text is in Persian Alphabet.
    /// It doesn't count the numbers and other non-alphabetical chars like " « , ،
    fn persian_percentage(&self) -> u8 {
        // First remove the non-alphabetic chars
        let string = self
            .as_ref()
            .chars()
            .into_iter()
            .filter(|c| c.is_alphabetic())
            .collect::<String>();
        let len = string.chars().count();

        if len == 0 {
            return 100;
        }

        let persian_chars = HAS_PERSIAN_CHAR.captures_iter(&string).count();
        (persian_chars * 100 / len) as u8
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
        assert!("گفت: «سلام»".is_persian_str());
        assert!(!"ok this is text with".is_persian_str());
        assert!(!"阴阳".is_persian_str());
        assert!(!"Hello".is_persian_str());
        assert!("سلام 😛".is_persian_str());
    }

    #[test]
    fn persian_percentage_test() {
        assert_eq!("سلام".persian_percentage(), 100);
        assert_eq!("گفت: «سلام»".persian_percentage(), 100);
        assert_eq!("ok this is text with".persian_percentage(), 0);
        assert_eq!("ok this is text with ص".persian_percentage(), 5);
        assert_eq!("阴阳".persian_percentage(), 0);
        assert_eq!("me من".persian_percentage(), 50);
        assert_eq!("阴阳 «من»".persian_percentage(), 50);
    }
}
