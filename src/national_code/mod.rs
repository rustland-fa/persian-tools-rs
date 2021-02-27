use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NATIONAL_CODE_REGEX: Regex = Regex::new(r"^\d{10}$").unwrap();
}

/// Takes a string and check if it's a valid Iranian national code or not.
pub fn is_iranian_national_code_valid<T: AsRef<str>>(code: T) -> bool {
    let code = code.as_ref();
    if !NATIONAL_CODE_REGEX.is_match(code) {
        return false;
    }
    if let Ok(num) = code[3..].parse::<u32>() {
        if num == 0 {
            return false;
        }
    }
    let last_index = code[9..].parse::<u32>().unwrap();
    let mut sum: u32 = 0;
    for i in 0..9 {
        sum += code[i..i + 1].parse::<u32>().unwrap() * (10 - i) as u32;
    }
    sum %= 11;
    (sum < 2 && last_index == sum) || (sum >= 2 && last_index == 11 - sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_iranian_national_code_test() {
        assert!(is_iranian_national_code_valid("3020588391"));
        assert!(!is_iranian_national_code_valid("3020588392"));
    }

    #[test]
    fn regex_test() {
        assert!(NATIONAL_CODE_REGEX.is_match("1234567890"));
        assert!(!NATIONAL_CODE_REGEX.is_match("123456789"));
        assert!(!NATIONAL_CODE_REGEX.is_match("123456789a"));
        assert!(!NATIONAL_CODE_REGEX.is_match("12345678911"));
    }
}
