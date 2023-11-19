use crate::utils::impl_trait_for_string_types;

pub trait NationalCode: AsRef<str> {
    /// Takes a string and check if it's a valid Iranian national code or not.
    fn is_valid_national_code(&self) -> bool {
        let code = self.as_ref();
        if code.len() != 10 { return false; }

        let digits: Vec<u32> = code.chars().map_while(|c| c.to_digit(10)).collect();
        if digits.len() != 10 { return false; }
        
        let last = digits[9];
        let sum = (0..9).map(|x| digits[x] * (10 - x) as u32).sum::<u32>() % 11;

        (sum < 2 && last == sum) || (last + sum == 11)
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
