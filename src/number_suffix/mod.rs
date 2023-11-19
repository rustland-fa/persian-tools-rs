use crate::utils::impl_trait_for_string_types;

static ORDINAL_SUFFIX: [&str; 7] = ["ام", "اُم", "ا", "اُ", "امین", "اُمین", "ین"];

/// Set of helpers to add ordinal suffixes to Persian numbers.
pub trait NumberSuffix: AsRef<str> {
    /// Add ordinal suffix to numbers.
    /// For example, will convert Persian text of "Panj" to "Panjom"
    fn add_ordinal_suffix_short(&self) -> String {
        let mut number = self.as_ref().trim().to_string();

        if !number.is_empty() {
            if number.ends_with('ی') {
                number.push_str("‌اُم"); // it includes a ZWNJ char!
            } else if number.ends_with("سه") {
                // remove the last char
                number.pop();
                number.push_str("وم");
            } else {
                number.push('م');
            }
        }

        number
    }

    /// Add ordinal suffix to numbers.
    /// For example, will convert Persian text of "Panj" to "Panjomin"
    fn add_ordinal_suffix_long(&self) -> String {
        let mut number = self.as_ref().to_string();

        if !number.is_empty() {
            number = number.add_ordinal_suffix_short();
            number.push_str("ین");
        }

        number
    }

    fn remove_ordinal_suffix(&self) -> String {
        let mut number = self.as_ref().to_string();
        if !number.is_empty() {
            for suffix in ORDINAL_SUFFIX {
                while number.ends_with(suffix) {
                    number.replace_range(number.len() - suffix.len().., "");
                }
            }

            if number.ends_with("سوم") {
                number = number.replace("سوم", "سه");
            } else if number.ends_with('م') {
                number.pop();
            } else if number.eq("اول") {
                number = "یک".to_string();
            }
            // U+200C is Zero-width non-joiner
            while number.ends_with(['\u{200c}', ' ']) {
                number.pop();
            }
        }

        number
    }
}

impl_trait_for_string_types!(NumberSuffix);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_ordinal_suffix_short_test() {
        assert_eq!("چهل و سه".add_ordinal_suffix_short(), "چهل و سوم");
        assert_eq!("چهل و پنج".add_ordinal_suffix_short(), "چهل و پنجم");
        assert_eq!("سی".add_ordinal_suffix_short(), "سی‌اُم");
        assert_eq!("یک".add_ordinal_suffix_short(), "یکم");
        assert_eq!("".add_ordinal_suffix_short(), "");
    }

    #[test]
    fn add_ordinal_suffix_long_test() {
        assert_eq!("چهل و سه".add_ordinal_suffix_long(), "چهل و سومین");
        assert_eq!("چهل و پنج".add_ordinal_suffix_long(), "چهل و پنجمین");
        assert_eq!("سی".add_ordinal_suffix_long(), "سی‌اُمین");
        assert_eq!("یک".add_ordinal_suffix_long(), "یکمین");
        assert_eq!("".add_ordinal_suffix_long(), "");
    }

    #[test]
    fn remove_ordinal_suffix_test() {
        assert_eq!("چهل و سوم".remove_ordinal_suffix(), "چهل و سه");
        assert_eq!("چهل و سومین".remove_ordinal_suffix(), "چهل و سه");
        assert_eq!("چهل و پنجم".remove_ordinal_suffix(), "چهل و پنج");
        assert_eq!("چهل و پنجمین".remove_ordinal_suffix(), "چهل و پنج");
        assert_eq!("سی‌اُم".remove_ordinal_suffix(), "سی");
        assert_eq!("سی‌اُمین".remove_ordinal_suffix(), "سی");
        assert_eq!("یکم".remove_ordinal_suffix(), "یک");
        assert_eq!("یکمین".remove_ordinal_suffix(), "یک");
        assert_eq!("اول".remove_ordinal_suffix(), "یک");
        assert_eq!("اولین".remove_ordinal_suffix(), "یک");
        assert_eq!("".remove_ordinal_suffix(), "");
    }
}
