use std::ops::RangeInclusive;

use crate::utils::impl_trait_for_string_types;

static HAS_PERSIAN_CHAR: RangeInclusive<char> = '\u{0600}'..='\u{06FF}';

/// Set of helpers for manipulating Persian text.
pub trait PersianContent: AsRef<str> {
    /// Checks if a text has at least a Persian char in it.
    fn has_persian_char(&self) -> bool {
        self.as_ref().chars().any(|c| HAS_PERSIAN_CHAR.contains(&c))
    }

    /// Checks if a text is in Persian.
    fn is_persian_str(&self) -> bool {
        self.as_ref()
            .chars()
            .filter(|c| c.is_alphabetic()) // First remove the non-alphabetic chars
            .all(|c| c.is_ascii_punctuation() || HAS_PERSIAN_CHAR.contains(&c))
    }

    /// Calculates how much of the text is in Persian Alphabet.
    /// It doesn't count the numbers and other non-alphabetical chars like " « , ،
    fn persian_percentage(&self) -> u8 {
        let (persian_chars_len, len) =
            self.as_ref()
                .chars()
                .fold((0u32, 0u32), |(mut pc, mut len), c| {
                    if c.is_alphabetic() {
                        if HAS_PERSIAN_CHAR.contains(&c) {
                            pc += 1;
                        }
                        len += 1;
                    }
                    (pc, len)
                });

        (persian_chars_len * 100).checked_div(len).unwrap_or(100) as u8
    }
}

impl_trait_for_string_types!(PersianContent);

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
        assert_eq!("".persian_percentage(), 100);
        assert_eq!("سلام".persian_percentage(), 100);
        assert_eq!("گفت: «سلام»".persian_percentage(), 100);
        assert_eq!("ok this is text with".persian_percentage(), 0);
        assert_eq!("ok this is text with ص".persian_percentage(), 5);
        assert_eq!("阴阳".persian_percentage(), 0);
        assert_eq!("me من".persian_percentage(), 50);
        assert_eq!("阴阳 «من»".persian_percentage(), 50);
    }
}
