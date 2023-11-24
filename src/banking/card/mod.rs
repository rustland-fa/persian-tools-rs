use std::str::FromStr;

pub use bank_codes_table::BANK_CODE_TABLE;

mod bank_codes_table;

#[derive(Debug, PartialEq, Eq)]
pub struct BankCardNumber(u64);

impl BankCardNumber {
    /// Get the bank name from card number.
    pub fn get_bank_name(&self) -> Option<&'static str> {
        let bank_id = (self.0 / 10u64.pow(10)) as u32;
        BANK_CODE_TABLE.get(&bank_id).copied()
    }
}

impl TryFrom<u64> for BankCardNumber {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value.checked_ilog10().unwrap_or(0) + 1 != 16 {
            return Err("A valid card number should be 16 in length".into());
        }

        if value == 0 {
            return Err("Digits of a valid card shouldn't add to 0".into());
        }

        if !is_card_valid(value) {
            return Err("Card is invalid".into());
        }

        Ok(Self(value))
    }
}

impl FromStr for BankCardNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 16 {
            return Err("A valid card number should be 16 in length".into());
        }

        let num = s.parse::<u64>().map_err(|e| e.to_string())?;

        if num == 0 {
            return Err("Digits of a valid card shouldn't add to 0".into());
        }

        if !is_card_valid(num) {
            return Err("Card is invalid".into());
        }

        Ok(Self(num))
    }
}

fn is_card_valid(num: u64) -> bool {
    get_digits(num)
        .into_iter()
        .enumerate()
        .map(|(idx, x)| {
            let mut sub_digit = x * if idx % 2 == 0 { 2 } else { 1 };
            if sub_digit > 9 {
                sub_digit -= 9
            }
            sub_digit as u32
        })
        .sum::<u32>()
        % 10
        == 0
}

fn get_digits(num: u64) -> Vec<u8> {
    (0..(num.checked_ilog10().unwrap_or(0) + 1) as u8)
        .rev()
        .map(|n| ((num / 10u64.pow(n as u32)) % 10) as u8)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_valid_card_number_test() {
        assert!(BankCardNumber::from_str("9999999999999999").is_err());
        assert!(BankCardNumber::try_from(9999999999999999).is_err());
        assert!(BankCardNumber::from_str("1234567890111213").is_err());
        assert!(BankCardNumber::try_from(1234567890111213).is_err());
        assert!(BankCardNumber::from_str("abcdefghi0111213").is_err());
        assert!(BankCardNumber::from_str("6395991167965611").is_err());
        assert!(BankCardNumber::from_str("6395991167965615").is_ok());
        assert!(BankCardNumber::try_from(6395991167965615).is_ok());
    }

    #[test]
    fn get_bank_name_from_card_number_test() {
        assert_eq!(
            Some("بانک قوامین"),
            BankCardNumber::from_str("6395991167965615")
                .unwrap()
                .get_bank_name()
        );
        assert_eq!(
            Some("بانک کشاورزی"),
            BankCardNumber::from_str("6037701689095443")
                .unwrap()
                .get_bank_name()
        );
        assert_eq!(
            Some("بانک سامان"),
            BankCardNumber::from_str("6219861034529007")
                .unwrap()
                .get_bank_name()
        );
        assert_eq!(
            Err("A valid card number should be 16 in length".to_owned()),
            BankCardNumber::from_str("")
        );

        assert_eq!(
            Some("بانک قوامین"),
            BankCardNumber::from_str("6395991167965615")
                .unwrap()
                .get_bank_name()
        );
        assert_eq!(
            Some("بانک قوامین"),
            BankCardNumber::try_from(6395991167965615)
                .unwrap()
                .get_bank_name()
        );
    }
}
