pub fn verify_card_number<T: AsRef<str>>(number: T) -> bool {
    let number = number.as_ref();
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_card_number_test() {
        let result = verify_card_number("1234567890111213");
        assert_eq!(result, false);
        let result = verify_card_number("abcdefghi0111213");
        assert_eq!(result, false);
        let result = verify_card_number("6395991167965611");
        assert_eq!(result, false);
        let result = verify_card_number("6395991167965615");
        assert!(result);
    }
}
