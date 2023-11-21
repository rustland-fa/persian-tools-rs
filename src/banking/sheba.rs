use crate::{banking::sheba_table::ShebaInfo, utils::impl_trait_for_string_types};

use crate::banking::sheba_table::SHEBA_CODE_TABLE;

pub trait ShebaNumber: AsRef<str> {
    fn is_valid_sheba_code(&self) -> bool {
        self.iso_7064_mod_97_10().is_ok_and(|i| i == 1)
    }

    fn get_sheba_info(&self) -> Option<ShebaInfo> {
        if !self.is_valid_sheba_code() {
            return None;
        }

        let digits = self.as_ref();
        SHEBA_CODE_TABLE
            .get(&&digits[4..7])
            .map(|sc| sc.process(digits))
    }

    fn iso_7064_mod_97_10(&self) -> crate::Result<i32> {
        let sheba_code = self.as_ref();
        // check if sheba is in valid format (^IR[0-9]{24}$)
        if !(sheba_code.len() == 26
            && sheba_code.starts_with("IR")
            && sheba_code.chars().skip(2).all(|c| c.is_ascii_digit()))
        {
            return Err("invalid sheba code".into());
        }

        let d1 = sheba_code.as_bytes()[0] - 65 + 10;
        let d2 = sheba_code.as_bytes()[1] - 65 + 10;
        let mut remainder = format!("{}{}{}{}", &sheba_code[4..], d1, d2, &sheba_code[2..4]);
        let mut block;
        loop {
            let len = remainder.len();
            if len <= 2 {
                break;
            }
            let pos = if len > 9 { 9 } else { len };
            block = &remainder[..pos];
            remainder = format!("{}{}", block.parse::<i32>()? % 97, &remainder[pos..]);
        }
        Ok(remainder.parse::<i32>()? % 97)
    }
}

impl_trait_for_string_types!(ShebaNumber);

#[cfg(test)]
mod sheba_code {
    use crate::banking::sheba_table::{
        process_parsian, process_pasargad, process_shahr, ShebaAccountNumber,
    };

    use super::*;

    #[test]
    fn sheba_code_validate() {
        assert!("IR210180000000009190404878".is_valid_sheba_code());
        assert!(!"123332132131432498654433".is_valid_sheba_code());
        assert!(!"IR1233321321314324986544323222".is_valid_sheba_code());
        assert!(!"IR1233321222".is_valid_sheba_code());
    }

    #[test]
    fn sheba_code_info() {
        assert!("IR210180000000009190404878".get_sheba_info().is_some());
        assert!("IR012345678901234567890123".get_sheba_info().is_none());
        assert!("IR012345678A01234567890123".get_sheba_info().is_none());

        let sheba_info_pasargad = ShebaInfo {
            code: "057",
            nickname: "pasargad",
            name: "Pasargad Bank",
            persian_name: "بانک پاسارگاد",
            account_number: Some(ShebaAccountNumber {
                normal: "220800134473701".to_owned(),
                formatted: "220-800-13447370-1".to_owned(),
            }),
            process: Some(process_pasargad),
        };
        let sheba_info_shahr = ShebaInfo {
            code: "061",
            nickname: "shahr",
            name: "City Bank",
            persian_name: "بانک شهر",
            account_number: Some(ShebaAccountNumber {
                normal: "700796858044".to_owned(),
                formatted: "700796858044".to_owned(),
            }),
            process: Some(process_shahr),
        };
        let sheba_info_parsian = ShebaInfo {
            code: "054",
            nickname: "parsian",
            name: "Parsian Bank",
            persian_name: "بانک پارسیان",
            account_number: Some(ShebaAccountNumber {
                normal: "020817909002".to_owned(),
                formatted: "002-00817909-002".to_owned(),
            }),
            process: Some(process_parsian),
        };

        assert_eq!(
            "IR550570022080013447370101".get_sheba_info(),
            Some(sheba_info_pasargad)
        );
        assert_eq!(
            "IR790610000000700796858044".get_sheba_info(),
            Some(sheba_info_shahr)
        );
        assert_eq!(
            "IR820540102680020817909002".get_sheba_info(),
            Some(sheba_info_parsian)
        );
    }
}
