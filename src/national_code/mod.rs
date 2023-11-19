use crate::utils::impl_trait_for_string_types;

pub trait NationalCode: AsRef<str> {
    /// Takes a string and check if it's a valid Iranian national code or not.
    fn is_valid_national_code(&self) -> bool {
        let code = self.as_ref();
        if !(code.len() == 10 && code.chars().all(|c| c.is_ascii_digit())) {
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
}

impl_trait_for_string_types!(NationalCode);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_valid_national_code_test() {
        assert!("3020588391".is_valid_national_code());
        assert!(!"3020588392".is_valid_national_code());
    }
}
