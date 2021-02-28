pub mod landline;
pub mod mobile;

#[cfg(test)]
mod tests {
    use super::{landline::*, mobile::*};

    #[test]
    fn is_valid_mobile_number_test_1() {
        assert!("09398254166".is_valid_mobile_number())
    }

    #[test]
    fn is_valid_mobile_number_test_2() {
        assert!("+989398254166".is_valid_mobile_number())
    }

    #[test]
    fn is_valid_landline_number_test() {
        assert!("03434144188".is_valid_landline_number());
        assert!(!"0343414418".is_valid_landline_number());
    }

    #[test]
    fn get_prefix_landline_number_test() {
        assert_eq!("03498254166".get_prefix_landline_number().unwrap(), "034");
        assert_eq!("+983498254166".get_prefix_landline_number().unwrap(), "034");
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
    }
}
