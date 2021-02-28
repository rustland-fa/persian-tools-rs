/// Set of helpers to add ordinal suffixes to Persian numbers.
trait NumberSuffix: AsRef<str> {
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
}

impl NumberSuffix for String {}
impl NumberSuffix for str {}

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
}
