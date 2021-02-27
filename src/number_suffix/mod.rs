/// Add ordinal suffix to numbers.
/// For example, will convert "Panj" to "Panjom"
pub fn add_ordinal_suffix_short<T: AsRef<str>>(number: T) -> String {
    let mut number = number.as_ref().trim().to_string();

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
/// For example, will convert "Panj" to "Panjomin"
pub fn add_ordinal_suffix_long<T: AsRef<str>>(number: T) -> String {
    let mut number = number.as_ref().to_string();

    if !number.is_empty() {
        number = add_ordinal_suffix_short(number);
        number.push_str("ین");
    }

    number
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_ordinal_suffix_short_test() {
        assert_eq!(add_ordinal_suffix_short("چهل و سه"), "چهل و سوم");
        assert_eq!(add_ordinal_suffix_short("چهل و پنج"), "چهل و پنجم");
        assert_eq!(add_ordinal_suffix_short("سی"), "سی‌اُم");
        assert_eq!(add_ordinal_suffix_short("یک"), "یکم");
        assert_eq!(add_ordinal_suffix_short(""), "");
    }

    #[test]
    fn add_ordinal_suffix_long_test() {
        assert_eq!(add_ordinal_suffix_long("چهل و سه"), "چهل و سومین");
        assert_eq!(add_ordinal_suffix_long("چهل و پنج"), "چهل و پنجمین");
        assert_eq!(add_ordinal_suffix_long("سی"), "سی‌اُمین");
        assert_eq!(add_ordinal_suffix_long("یک"), "یکمین");
        assert_eq!(add_ordinal_suffix_long(""), "");
    }
}
