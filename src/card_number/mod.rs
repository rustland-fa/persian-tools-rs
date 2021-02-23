pub fn verify_card_number<T: AsRef<str>>(number: T) -> bool {
    let number = number.as_ref();
    let len = number.len();
    if len < 16 {
        return false;
    }
    match number[1..10].parse::<u32>() {
        Ok(p) => {
            if p == 0 {
                return false;
            }
        }
        Err(_) => {
            return false;
        }
    }
    match number[10..16].parse::<u32>() {
        Ok(p) => {
            if p == 0 {
                return false;
            }
        }
        Err(_) => {
            return false;
        }
    }
    let mut sum = 0;
    let mut even;
    let mut sub_digit;
    for i in 0..16 {
        even = if i % 2 == 0 { 2 } else { 1 };
        sub_digit = match number[i..i + 1].parse::<i32>() {
            Ok(d) => d * even,
            Err(_) => {
                return false;
            }
        };
        sum += if sub_digit > 9 {
            sub_digit - 9
        } else {
            sub_digit
        };
    }
    return sum % 10 == 0;
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn verify_card_number_test() {
        let result = verify_card_number("1234567890111213");
        assert_eq!(result, false);
        let result = verify_card_number("6395991167965611");
        assert_eq!(result, false);
    }
}
