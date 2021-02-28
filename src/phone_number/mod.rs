pub mod landline;
pub mod mobile;

#[cfg(test)]
mod tests {
    use super::{landline::*, mobile::*};

    #[test]
    fn is_valid_mobile_number_test() {
        let result = is_valid_mobile_number("09398254166");
        assert_eq!(result, true);
    }

    #[test]
    fn is_valid_mobile_number_test2() {
        let result = is_valid_mobile_number("093982541");
        assert_eq!(result, false);
    }

    #[test]
    fn is_valid_landline_number_test() {
        let result = is_valid_landline_number("03434144166");
        assert_eq!(result, true);
        let result = is_valid_landline_number("0343414412");
        assert_eq!(result, false);
    }

    #[test]
    fn get_prefix_landline_number_test() {
        let result = get_prefix_landline_number("03498254166").unwrap();
        assert_eq!(&result, "034");
        let result = get_prefix_landline_number("+983498254166").unwrap();
        assert_eq!(&result, "034");
    }

    #[test]
    fn get_operator_name_from_mobile_number_test() {
        let result = get_operator_name_from_mobile_number("09324343312");
        assert_eq!(result.unwrap().unwrap(), IranMobileOperator::Taliya);
        let result = get_operator_name_from_mobile_number("+989324343312");
        assert_eq!(result.unwrap().unwrap(), IranMobileOperator::Taliya);
        let result = get_operator_name_from_mobile_number("989324343312");
        assert_eq!(result.unwrap().unwrap(), IranMobileOperator::Taliya);
        let result = get_operator_name_from_mobile_number("09324343312");
        assert_eq!(result.unwrap().unwrap(), IranMobileOperator::Taliya);
        let result = get_operator_name_from_mobile_number("09124343312");
        assert_eq!(result.unwrap().unwrap(), IranMobileOperator::MCI);
    }
}
