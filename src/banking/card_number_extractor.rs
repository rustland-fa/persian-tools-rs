use std::str::Chars;

use super::Banking;
use crate::digit::Digit;

static CARD_SEPERATORS: [char; 4] = ['-', '_', '*', '.'];

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

pub struct CardNumberExtractor<'a> {
    chars: Chars<'a>,
    index: usize,
    options: ExtractCardNumberOptions,
}

impl<'a> CardNumberExtractor<'a> {
    pub fn new(s: &'a str, options: ExtractCardNumberOptions) -> Self {
        Self { chars: s.chars(), index: 0, options }
    }

    /// Collect all card number to a vector
    pub fn to_vec(self) -> Vec<CardNumber> {
        self.collect()
    }

    fn get_next_card(&mut self) -> Option<String> {
        let mut len = 0;
        let mut base = String::with_capacity(20);

        loop {
            let ch = self.chars.next()?;

            match CharType::new(&ch) {
                CharType::Digit => {
                    base.push(ch);
                    len += 1;

                    if len == 16 {
                        return Some(base);
                    }
                },
                CharType::Seperator => base.push(ch),
                CharType::Other => {
                    base.clear();
                    len = 0;
                },
            }
        }
    }
}

impl<'a> Iterator for CardNumberExtractor<'a> {
    type Item = CardNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let mut is_valid;

        // get card-number, filter the card if needed
        let (base, pure) = loop {
            let base = self.get_next_card()?;
            let mut pure = base.replace(CARD_SEPERATORS, "");

            self.index += 1;

            if pure.have_non_en_digit() {
                // if there is any non english digit replace them with english digits
                pure = pure.digits_to_en();
            }
            
            is_valid = self.options.check_validation.then(|| pure.is_valid_bank_card_number());

            if !self.options.filter_valid_card_numbers || is_valid.unwrap_or(true) {
                break (base, pure);
            }
        };
        
        let bank_name = if self.options.detect_bank_name {
            pure.get_bank_name_from_card_number()
        } else {
            None
        };

        Some(CardNumber {
            base,
            is_valid,
            bank_name,
            pure,
            index: self.index,
        })
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
            c if CARD_SEPERATORS.contains(c) => Self::Seperator,
            _ => Self::Other,
        }
    }
}

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

        let options = ExtractCardNumberOptions {
            check_validation: false,
            ..Default::default()
        };
        assert_eq!(
            cards,
            CardNumberExtractor::new(string, options).to_vec()
        );

        // Should find and format the Card-Number into Text that includes Persian & English digits
        {
            let string = "شماره کارتم رو برات نوشتم: ۵۰۲۲-2910-7۰۸۷-۳۴۶۶";

            let card = create_card!("۵۰۲۲-2910-7۰۸۷-۳۴۶۶", "5022291070873466", 1);

            let options = ExtractCardNumberOptions {
                check_validation: false,
                ..Default::default()
            };
            assert_eq!(
                vec![card],
                CardNumberExtractor::new(string, options).to_vec()
            );
        }

        // Should validate extract card-numbers
        let cards = vec![
            create_card!("6219-8610-3452-9007", "6219861034529007", 1, true),
            create_card!("5022291070873466", "5022291070873466", 2, true),
            create_card!("۵۰۲۲۲۹۱۰۸۱۸۷۳۴۶۶", "5022291081873466", 3, false),
            create_card!("۵۰۲۲-۲۹۱۰-۷۰۸۷-۳۴۶۶", "5022291070873466", 4, true),
        ];

        let options = ExtractCardNumberOptions {
            check_validation: true,
            filter_valid_card_numbers: false,
            ..Default::default()
        };

        assert_eq!(
            cards,
            CardNumberExtractor::new(string, options).to_vec()
        );

        // Should return only valid card-numbers
        let cards = vec![
            create_card!("6219-8610-3452-9007", "6219861034529007", 1, true),
            create_card!("5022291070873466", "5022291070873466", 2, true),
            create_card!("۵۰۲۲-۲۹۱۰-۷۰۸۷-۳۴۶۶", "5022291070873466", 4, true),
        ];

        let options = ExtractCardNumberOptions {
            check_validation: true,
            filter_valid_card_numbers: true,
            ..Default::default()
        };

        assert_eq!(
            cards,
            CardNumberExtractor::new(string, options).to_vec()
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
            CardNumberExtractor::new(string, ExtractCardNumberOptions::all()).to_vec(),
        );
    }
}
