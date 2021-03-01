pub mod landline;
pub mod mobile;

#[cfg(test)]
mod tests {
    use super::{landline::*, mobile::*};
    use crate::province::IranProvince;

    #[test]
    fn is_valid_mobile_number_test() {
        assert!("09398254166".is_valid_mobile_number());
        assert!("+989398254166".is_valid_mobile_number());
        assert!(!"+98939825416621121121122133313".is_valid_mobile_number());
    }

    #[test]
    fn is_valid_landline_number_test() {
        assert!("03434144188".is_valid_landline_number());
        assert!(!"0343414418".is_valid_landline_number());
        assert!(!"034341441810000000000000000023323232".is_valid_landline_number());
    }

    #[test]
    fn get_prefix_landline_number_test() {
        assert_eq!("03498254166".get_prefix_landline_number().unwrap(), "034");
        assert_eq!("+983498254166".get_prefix_landline_number().unwrap(), "034");
        assert!("+98".get_operator_name_from_mobile_number().is_err());
    }

    #[test]
    fn get_province_from_landline_number_test() {
        assert_eq!(
            "03498254166"
                .get_province_from_landline_number()
                .unwrap()
                .unwrap(),
            IranProvince::Kerman
        );
        assert_eq!(
            "+982198254166"
                .get_province_from_landline_number()
                .unwrap()
                .unwrap(),
            IranProvince::Tehran
        );
        assert!("+98999999999".get_province_from_landline_number().is_err());
    }

    #[test]
    fn get_operator_name_from_mobile_number_test() {
        assert_eq!(
            "09324341133"
                .get_operator_name_from_mobile_number()
                .unwrap()
                .unwrap(),
            IranMobileOperator::Taliya
        );
        assert_eq!(
            "+989324341133"
                .get_operator_name_from_mobile_number()
                .unwrap()
                .unwrap(),
            IranMobileOperator::Taliya
        );
        assert_eq!(
            "+989134341133"
                .get_operator_name_from_mobile_number()
                .unwrap()
                .unwrap(),
            IranMobileOperator::MCI
        );
        assert!("+98999999999"
            .get_operator_name_from_mobile_number()
            .is_err());
    }
}
