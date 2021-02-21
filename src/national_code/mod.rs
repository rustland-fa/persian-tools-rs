pub fn verify_iranian_national_code<T: AsRef<str>>(code: T) -> bool {
    let len = code.as_ref().len();
    let code = format!("0000{}", code.as_ref());
    let code = code.as_str()[(len + 4 - 10)..].to_string();
    if let Ok(num) = code.as_str()[3..].parse::<u32>() {
        if num == 0 {
            return false;
        }
    }
    let last_index = code.as_str()[9..].parse::<u32>().unwrap();
    let mut sum: u32 = 0;
    for i in 0..9 {
        sum += code.as_str()[i..i + 1].parse::<u32>().unwrap() * (10 - i) as u32;
    }
    sum = sum % 11;
    return (sum < 2 && last_index == sum) || (sum >= 2 && last_index == 11 - sum);
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn verify_iranian_national_code_test() {
        let result = verify_iranian_national_code("3020588391");
        assert_eq!(result, true);
    }
}
