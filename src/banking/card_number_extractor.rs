use super::Banking;
use crate::{digit::Digit, utils::impl_trait_for_string_types};

/// Card number information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardNumber {
    /// Base card-number
    pub base: String,
    /// Card-number without any extra character
    pub pure: String,
    /// Start Index of card-number (based on chars)
    pub index: usize,
    /// Card-numbers is valid or not
    pub is_valid: Option<bool>,
    /// bank name of card-number
    // TODO(saeid): maybe make this `Option<Option<..>>`?
    pub bank_name: Option<&'static str>,
}

/// Extract card number options
#[derive(Clone, Copy, Debug)]
pub struct ExtractCardNumberOptions {
    /// Check if every card-numbers is valid or not
    pub check_validation: bool,
    /// Detect Bank's name by extracted card-number
    pub detect_bank_name: bool,
    /// Return list of only valid card-numbers
    pub filter_valid_card_numbers: bool,
}

impl ExtractCardNumberOptions {
    /// Enable all `check_validation`, `detect_bank_name` and `filter_valid_card_numbers` options.
    pub fn all() -> Self {
        Self {
            check_validation: true,
            detect_bank_name: true,
            filter_valid_card_numbers: true,
        }
    }

    /// Disable all options.
    pub fn none() -> Self {
        Self {
            check_validation: false,
            detect_bank_name: false,
            filter_valid_card_numbers: false,
        }
    }
}

impl Default for ExtractCardNumberOptions {
    fn default() -> Self {
        Self {
            check_validation: true,
            detect_bank_name: false,
            filter_valid_card_numbers: true,
        }
    }
}

pub trait ExtractCardNumber: AsRef<str> {
    /// Extract all the card numbers.
    fn extract_card_numbers(&self, options: ExtractCardNumberOptions) -> Vec<CardNumber> {
        let digits = self.as_ref();
        let mut result = Vec::new();

        let mut len = 0;
        let mut base = String::with_capacity(20);
        let mut pure = String::with_capacity(16);
        for c in digits.chars() {
            match CharType::new(&c) {
                CharType::Digit => {
                    base.push(c);
                    pure.push(c);
                    len += 1;

                    // a valid iranian card-number have 16 digits
                    if len == 16 {
                        if pure.have_non_en_digit() {
                            // if there is any non english digit replace them with english digits
                            pure = pure.digits_to_en();
                        }

                        result.push(CardNumber {
                            base: base.clone(),
                            is_valid: options
                                .check_validation
                                .then(|| pure.is_valid_bank_card_number()),
                            bank_name: if options.detect_bank_name {
                                pure.get_bank_name_from_card_number()
                            } else {
                                None
                            },
                            pure: pure.clone(),
                            index: result.len() + 1,
                        });
                        // clear buffers and len after we pushed the information to result
                        base.clear();
                        pure.clear();
                        len = 0;
                    }
                }
                CharType::Seperator => base.push(c),
                CharType::Other => {
                    // clear buffers and len in case of unsupported character
                    base.clear();
                    pure.clear();
                    len = 0;
                }
            }
        }

        if options.filter_valid_card_numbers {
            result.retain(|c| c.is_valid.unwrap_or(true));
        }

        result
    }
}

enum CharType {
    Digit,
    Seperator,
    Other,
}

impl CharType {
    fn new(c: &char) -> Self {
        match c {
            '0'..='9' | '\u{0660}'..='\u{0669}' | '\u{06F0}'..='\u{06F9}' => Self::Digit,
            '-' | '_' | '*' | '.' => Self::Seperator,
            _ => Self::Other,
        }
    }
}

impl_trait_for_string_types!(ExtractCardNumber);

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! create_card {
        ($base:literal, $pure:literal, $index:literal) => {
            CardNumber {
                base: $base.to_owned(),
                pure: $pure.to_owned(),
                index: $index,
                is_valid: None,
                bank_name: None,
            }
        };
        ($base:literal, $pure:literal, $index:literal, $is_valid:literal) => {
            CardNumber {
                base: $base.to_owned(),
                pure: $pure.to_owned(),
                index: $index,
                is_valid: Some($is_valid),
                bank_name: None,
            }
        };
        ($base:literal, $pure:literal, $index:literal, $is_valid:literal, $bank_name:literal) => {
            CardNumber {
                base: $base.to_owned(),
                pure: $pure.to_owned(),
                index: $index,
                is_valid: Some($is_valid),
                bank_name: Some($bank_name),
            }
        };
    }

    #[test]
    fn extract_card_number_test() {
        // Should find and extract 4 Card Numbers
        let string = r#"شماره کارتم رو برات نوشتم:
        6219-8610-3452-9007
        اینم یه شماره کارت دیگه ای که دارم
        5022291070873466
        ۵۰۲۲۲۹۱۰۸۱۸۷۳۴۶۶
        ۵۰۲۲-۲۹۱۰-۷۰۸۷-۳۴۶۶"#;

        // Should find and extract 4 Card Numbers
        let cards = vec![
            create_card!("6219-8610-3452-9007", "6219861034529007", 1),
            create_card!("5022291070873466", "5022291070873466", 2),
            create_card!("۵۰۲۲۲۹۱۰۸۱۸۷۳۴۶۶", "5022291081873466", 3),
            create_card!("۵۰۲۲-۲۹۱۰-۷۰۸۷-۳۴۶۶", "5022291070873466", 4),
        ];

        assert_eq!(
            cards,
            string.extract_card_numbers(ExtractCardNumberOptions {
                check_validation: false,
                ..Default::default()
            })
        );

        // Should find and format the Card-Number into Text that includes Persian & English digits
        {
            let string = "شماره کارتم رو برات نوشتم: ۵۰۲۲-2910-7۰۸۷-۳۴۶۶";

            let card = create_card!("۵۰۲۲-2910-7۰۸۷-۳۴۶۶", "5022291070873466", 1);

            assert_eq!(
                vec![card],
                string.extract_card_numbers(ExtractCardNumberOptions {
                    check_validation: false,
                    ..Default::default()
                })
            );
        }

        // Should validate extract card-numbers
        let cards = vec![
            create_card!("6219-8610-3452-9007", "6219861034529007", 1, true),
            create_card!("5022291070873466", "5022291070873466", 2, true),
            create_card!("۵۰۲۲۲۹۱۰۸۱۸۷۳۴۶۶", "5022291081873466", 3, false),
            create_card!("۵۰۲۲-۲۹۱۰-۷۰۸۷-۳۴۶۶", "5022291070873466", 4, true),
        ];

        assert_eq!(
            cards,
            string.extract_card_numbers(ExtractCardNumberOptions {
                check_validation: true,
                filter_valid_card_numbers: false,
                ..Default::default()
            })
        );

        // Should return only valid card-numbers
        let cards = vec![
            create_card!("6219-8610-3452-9007", "6219861034529007", 1, true),
            create_card!("5022291070873466", "5022291070873466", 2, true),
            create_card!("۵۰۲۲-۲۹۱۰-۷۰۸۷-۳۴۶۶", "5022291070873466", 4, true),
        ];

        assert_eq!(
            cards,
            string.extract_card_numbers(ExtractCardNumberOptions {
                check_validation: true,
                filter_valid_card_numbers: true,
                ..Default::default()
            })
        );

        // Should detect Banks number for valid card-numbers
        let cards = vec![
            create_card!(
                "6219-8610-3452-9007",
                "6219861034529007",
                1,
                true,
                "بانک سامان"
            ),
            create_card!(
                "5022291070873466",
                "5022291070873466",
                2,
                true,
                "بانک پاسارگاد"
            ),
            create_card!(
                "۵۰۲۲-۲۹۱۰-۷۰۸۷-۳۴۶۶",
                "5022291070873466",
                4,
                true,
                "بانک پاسارگاد"
            ),
        ];

        assert_eq!(
            cards,
            string.extract_card_numbers(ExtractCardNumberOptions {
                check_validation: true,
                filter_valid_card_numbers: true,
                detect_bank_name: true
            })
        );
    }
}
