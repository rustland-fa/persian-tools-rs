pub mod landline;
pub mod mobile;

static NUMBER_PREFIX_STR: [&str; 4] = ["+98", "0", "98", "0098"];

fn get_num_skip_str(text: &str) -> usize {
    for prefix in NUMBER_PREFIX_STR {
        if text.starts_with(prefix) {
            return prefix.len();
        }
    }
    0
}

fn validate_num(num: u64) -> Option<u64> {
    let len = num.checked_ilog10().unwrap_or(0) + 1;
    match len {
        12 => {
            if num / 10u64.pow(10) == 98 {
                Some(num % 10u64.pow(10))
            } else {
                None
            }
        }
        10 => Some(num),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{landline::*, mobile::*};
    use crate::province::IranProvince;

    #[test]
    fn is_valid_mobile_number_test() {
        assert!(MobileNumber::from_str("09398254166").is_ok());
        assert!(MobileNumber::try_from(9398254166).is_ok());
        assert!(MobileNumber::from_str("+989398254166").is_ok());
        assert!(MobileNumber::try_from(989398254166).is_ok());
        assert!(MobileNumber::from_str("+98939825416621121121122133313").is_err());
    }

    #[test]
    fn is_valid_landline_number_test() {
        assert!(LandlineNumber::from_str("03434144188").is_ok());
        assert!(LandlineNumber::from_str("0343414418").is_err());
        assert!(LandlineNumber::from_str("034341441810000000000000000023323232").is_err());
    }

    #[test]
    fn get_prefix_landline_number_test() {
        assert_eq!(
            LandlineNumber::from_str("03498254166")
                .unwrap()
                .get_prefix_landline_number(),
            34
        );
        assert_eq!(
            LandlineNumber::from_str("+983498254166")
                .unwrap()
                .get_prefix_landline_number(),
            34
        );
        assert!(MobileNumber::from_str("+98").is_err());
    }

    #[test]
    fn get_province_from_landline_number_test() {
        assert_eq!(
            LandlineNumber::from_str("03498254166")
                .unwrap()
                .get_province()
                .unwrap(),
            IranProvince::Kerman
        );
        assert_eq!(
            LandlineNumber::from_str("+982198254166")
                .unwrap()
                .get_province()
                .unwrap(),
            IranProvince::Tehran
        );
        assert!(LandlineNumber::from_str("+98999999999").is_err());
    }

    #[test]
    fn get_operator_name_from_mobile_number_test() {
        assert_eq!(
            MobileNumber::from_str("09324341133")
                .unwrap()
                .get_operator_name()
                .unwrap(),
            IranMobileOperator::Taliya
        );
        assert_eq!(
            MobileNumber::from_str("+989324341133")
                .unwrap()
                .get_operator_name()
                .unwrap(),
            IranMobileOperator::Taliya
        );
        assert_eq!(
            MobileNumber::from_str("+989134341133")
                .unwrap()
                .get_operator_name()
                .unwrap(),
            IranMobileOperator::MCI
        );
        assert_eq!(
            MobileNumber::from_str("+989999048230")
                .unwrap()
                .get_operator_name()
                .unwrap(),
            IranMobileOperator::SamanTel
        );
        assert!(MobileNumber::from_str("+98999999999").is_err());
    }
}
