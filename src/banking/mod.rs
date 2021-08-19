use crate::banking::bank_codes_table::BANK_CODE_TABLE;
use crate::impl_trait_for_string_types;

pub mod bank_codes_table;
pub mod sheba;
pub mod sheba_table;

/// Set of helpers for the banking system of Iran.
pub trait Banking: AsRef<str> {
    /// Checks if the bank card number is valid or not.
    fn is_valid_bank_card_number(&self) -> bool {
        let number = self.as_ref();
        let len = number.len();
        if len < 16
            || number.parse::<u64>().is_err()
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

    /// Get the bank name from card number.
    fn get_bank_name_from_card_number(&self) -> String {
        let number = self.as_ref();
        if number.is_valid_bank_card_number() {
            return BANK_CODE_TABLE[&number[0..6]].name.to_string();
        }
        number.to_string()
    }
}

impl_trait_for_string_types!(Banking);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_valid_card_number_test() {
        assert!(!"9999999999999999".is_valid_bank_card_number());
        assert!(!"1234567890111213".is_valid_bank_card_number());
        assert!(!"abcdefghi0111213".is_valid_bank_card_number());
        assert!(!"6395991167965611".is_valid_bank_card_number());
        assert!("6395991167965615".is_valid_bank_card_number());
        assert!(Banking::is_valid_bank_card_number("6395991167965615"));

        let string = "6395991167965615".to_string();
        assert!(string.is_valid_bank_card_number());
        assert!(Banking::is_valid_bank_card_number(&string));
    }

    #[test]
    fn get_bank_name_from_card_number_test() {
        assert_eq!(
            "بانک قوامین",
            "6395991167965615".get_bank_name_from_card_number()
        );
        assert_eq!(
            "بانک کشاورزی",
            "6037701689095443".get_bank_name_from_card_number()
        );
        assert_eq!(
            "بانک سامان",
            "6219861034529007".get_bank_name_from_card_number()
        );
        assert_eq!("", "".get_bank_name_from_card_number());

        let string = "6395991167965615".to_string();
        assert_eq!("بانک قوامین", string.get_bank_name_from_card_number());
        assert_eq!(
            "بانک قوامین",
            Banking::get_bank_name_from_card_number(&string)
        );
    }
}
