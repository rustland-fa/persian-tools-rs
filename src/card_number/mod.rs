trait CardVerification: AsRef<str> {
    fn is_valid_card_number(&self) -> bool 
    {
        let number = self.as_ref();
        let len = number.len();
        if len < 16
            || number.chars().any(|c| !c.is_numeric())
            || number[1..10].parse::<u32>().unwrap() == 0
            || number[10..16].parse::<u32>().unwrap() == 0
        {
            return false;
        }

        let mut sum = 0;
        let mut even;
        let mut sub_digit;
        for i in 0..16 {
            even = if i % 2 == 0 { 2 } else { 1 };
            sub_digit = number[i..i + 1].parse::<i32>().unwrap() * even;
            sum += if sub_digit > 9 {
                sub_digit - 9
            } else {
                sub_digit
            };
        }
        sum % 10 == 0
    }
}

impl CardVerification for String {}
impl CardVerification for &str {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_card_number_test() {
        assert!(!"1234567890111213".is_valid_card_number());
        assert!(!"abcdefghi0111213".is_valid_card_number());
        assert!(!"6395991167965611".is_valid_card_number());
        assert!("6395991167965615".is_valid_card_number());
    }
}
