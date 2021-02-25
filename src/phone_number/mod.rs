pub mod landline;
pub mod mobile;

#[cfg(test)]
pub mod tests {
    use super::landline::*;
    use super::mobile::*;

    #[test]
    pub fn is_valid_mobile_number_test() {
        let result = is_valid_moblie_number("09398254166");
        assert_eq!(result, true);
    }
    #[test]
    pub fn get_prefix_mobile_number_test() {
        let result = get_prefix_mobile_number("09398254166").unwrap();
        assert_eq!(&result, "939");
        let result = get_prefix_mobile_number("+989398254166").unwrap();
        assert_eq!(&result, "939");
    }

    #[test]
    pub fn is_valid_landline_number_test() {
        let result = is_valid_landline_number("03434144166");
        assert_eq!(result, true);
    }

    #[test]
    pub fn get_prefix_landline_number_test() {
        let result = get_prefix_landline_number("03498254166").unwrap();
        assert_eq!(&result, "34");
        let result = get_prefix_landline_number("+983498254166").unwrap();
        assert_eq!(&result, "34");
    }
}
