use crate::{banking::sheba_table::ShebaInfo, utils::impl_trait_for_string_types};

use crate::banking::sheba_table::SHEBA_CODE_TABLE;

pub trait ShebaNumber: AsRef<str> {
    fn is_valid_sheba_code(&self) -> bool {
        self.iso_7064_mod_97_10().is_ok_and(|i| i == 1)
    }

    fn sheba_code_info(&self) -> Option<&ShebaInfo> {
        if !self.is_valid_sheba_code() {
            return None;
        }
        SHEBA_CODE_TABLE.get(&&self.as_ref()[4..7])
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
        assert!("IR210180000000009190404878".sheba_code_info().is_some())
    }
}
