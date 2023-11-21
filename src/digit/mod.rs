use crate::utils::*;
use std::convert::TryFrom;

/// Supported language variants.
#[derive(Clone, Copy)]
pub enum Lang {
    En,
    Fa,
    Ar,
}

/// Set of helpers to manipulate Persian (or Arabic!) digits.
pub trait Digit: AsRef<str> {
    const DIGITS: [[char; 10]; 3] = [
        ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
        ['۰', '۱', '۲', '۳', '۴', '۵', '۶', '۷', '۸', '۹'],
        ['٠', '١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩'],
    ];

    /// Takes a string that may contain some digits, and
    /// replaces the source language digits with the destination
    /// language digits.
    fn digits_convert(&self, from: Lang, to: Lang) -> String {
        let src = self.as_ref();
        src.chars()
            .map(
                |v| match Self::DIGITS[from as usize].iter().position(|&r| r == v) {
                    Some(v) => Self::DIGITS[to as usize][v],
                    None => v,
                },
            )
            .collect()
    }

    /// Takes a string that may contain English digits, and returns
    /// a string that represents the same digits but in Persian.
    fn digits_en_to_fa(&self) -> String {
        self.digits_convert(Lang::En, Lang::Fa)
    }

    /// Takes a string that may contain Persian digits, and returns
    /// a string that represents the same digits but in English.
    fn digits_fa_to_en(&self) -> String {
        self.digits_convert(Lang::Fa, Lang::En)
    }

    /// Takes a string that may contain Arabic digits, and returns
    /// a string that represents the same digits but in Persian.
    fn digits_ar_to_fa(&self) -> String {
        self.digits_convert(Lang::Ar, Lang::Fa)
    }

    /// Takes a string that may contain Arabic digits, and returns
    /// a string that represents the same digits but in English.
    fn digits_ar_to_en(&self) -> String {
        self.digits_convert(Lang::Ar, Lang::En)
    }

    /// Takes a string that may contain Persian or Arabic digits, and returns
    /// a string that represents the same digits but in English.
    fn digits_to_en(&self) -> String {
        self.digits_convert(Lang::Ar, Lang::En)
            .digits_convert(Lang::Fa, Lang::En)
    }

    /// Check if the string have any non english (arabic, persian) digits.
    fn have_non_en_digit(&self) -> bool {
        self.as_ref()
            .chars()
            .any(|c| Self::DIGITS.iter().skip(1).any(|d| d.contains(&c)))
    }
}

impl_trait_for_string_types!(Digit);

/// The multipliers of the persian number system, up to a billion.
pub static MULTIPLIERS: FixedMap<&str, u32> = create_fixed_map! {
    "هزار" => 1_000,
    "میلیون" => 1_000_000,
    "میلیارد" => 1_000_000_000,
};

/// Fixed numbers that we interpret at face-value, as-is, because they cannot be broken down into
/// smaller parts.
///
/// Includes [1-20], [30, 40, ..., 100], and [100, 200, ..., 900]
// TODO: probably move to another file, too much bloat here.
// TOOD: Is it 'nohsad' or 'noh sad'? 'haftsad or 'haft sad'?
pub static FACE_VALUE: FixedMap<&str, u16> = create_fixed_map! {
    "صفر" => 0,
    "یک" => 1,
    "دو" => 2,
    "سه" => 3,
    "چهار" => 4,
    "پنج" => 5,
    "شش" => 6,
    "هفت" => 7,
    "هشت" => 8,
    "نه" => 9,
    "ده" => 10,
    "یازده" => 11,
    "دوازده" => 12,
    "سیزده" => 13,
    "چهارده" => 14,
    "پانزده" => 15,
    "شانزده" => 16,
    "هفده" => 17,
    "هجده" => 18,
    "نوزده" => 19,
    "بیست" => 20,
    "سی" => 30,
    "چهل" => 40,
    "پنجاه" => 50,
    "شصت" => 60,
    "هفتاد" => 70,
    "هشتاد" => 80,
    "نود" => 90,
    "صد" => 100,
    "دویست" => 200,
    "سیصد" => 300,
    "جهارصد" => 400,
    "پانصد" => 500,
    "ششصد" => 600,
    "هفتصد" => 700,
    "هشتصد" => 800,
    "نهصد" => 900,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TokenType {
    Multiplier(u32),
    FaceValue(u16),
}

impl TryFrom<&str> for TokenType {
    type Error = &'static str;

    fn try_from(token: &str) -> Result<Self, Self::Error> {
        if let Some(v) = FACE_VALUE.get(&token) {
            Ok(TokenType::FaceValue(*v))
        } else if let Some(m) = MULTIPLIERS.get(&token) {
            Ok(TokenType::Multiplier(*m))
        } else {
            Err("Unsupported token")
        }
    }
}

/// Extension trait for conversion of number strings in written format into numbers.
///
/// See [`words_to_number`] for more details.
pub trait WordsToNumber: AsRef<str> {
    /// Convert `&self` into a number with given type `N`.
    ///
    /// Needless to say, the resulting number must fit into `N`.
    ///
    /// Supports only values from [0, 999_999_999_999].
    ///
    /// The given string may contain only tokens being equal to either of [`FACE_VALUE`] or
    /// [`MULTIPLIERS`], or the special case of "و". Any other token will lead to a parse error.
    // IMPLEMENTATION NOTE: we follow the semantics of a basic stack-machine: Any face-value type
    // will be accumulated, until a multiplier is seen, at which point, it is multiplied into the
    // accumulated face-values, and we reset.
    fn words_to_number<
        N: num_traits::Zero
            + num_traits::One
            + num_traits::CheckedMul
            + num_traits::CheckedAdd
            + std::convert::TryFrom<u32>
            + std::convert::TryFrom<u16>
            + std::fmt::Debug
            + Copy,
    >(
        &self,
    ) -> crate::Result<N> {
        // TODO: ^^ maybe make a module-level Result alias.
        const CANT_CONVERT: &str = "Given number does not fit in the provided`N`";

        let parsed = self
            .as_ref()
            .split(' ')
            .filter(|t| *t != "و")
            .map(TokenType::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let mut final_value: N = num_traits::Zero::zero();
        let mut intermediary_value: N = num_traits::Zero::zero();

        let check_add = |lhs: N, rhs: N| lhs.checked_add(&rhs).ok_or(CANT_CONVERT);

        let checked_mul = |lhs: N, rhs: N| lhs.checked_mul(&rhs).ok_or(CANT_CONVERT);
        let mut last: Option<TokenType> = None;

        for t in parsed {
            match t {
                TokenType::FaceValue(v) => {
                    let v_n: N = v.try_into().map_err(|_| CANT_CONVERT)?;

                    intermediary_value = check_add(intermediary_value, v_n)?;
                    last = Some(TokenType::FaceValue(v));
                }
                TokenType::Multiplier(m) => {
                    if last.map_or(false, |last| matches!(last, TokenType::Multiplier(_))) {
                        return Err("Incorrect format: two multipliers in a row".into());
                    }

                    // a bit of helper: if this is the first iteration, you can omit a 'یک' and we
                    // replace it here.
                    if last.is_none() {
                        intermediary_value = num_traits::One::one();
                    }

                    let m_n: N = m.try_into().map_err(|_| CANT_CONVERT)?;
                    intermediary_value = checked_mul(intermediary_value, m_n)?;
                    final_value = check_add(final_value, intermediary_value)?;

                    intermediary_value = num_traits::Zero::zero();
                    last = Some(TokenType::Multiplier(m));
                }
            }
        }

        if let Some(TokenType::FaceValue(_)) = last {
            final_value = final_value
                .checked_add(&intermediary_value)
                .ok_or(CANT_CONVERT)?;
        }

        Ok(final_value)
    }
}

impl_trait_for_string_types!(WordsToNumber);

#[cfg(test)]
mod word_to_number {
    use super::*;

    #[test]
    fn words_to_number_basic() {
        // a random example to begin with.
        assert_eq!(
            "هفتصد و بیست و یک هزار و دویست و بیست و یک"
                .words_to_number::<u32>()
                .unwrap(),
            721221,
        );
    }

    #[test]
    fn face_value_only() {
        // face value only.
        assert_eq!("یک".words_to_number::<u32>().unwrap(), 1);
        assert_eq!("یازده".words_to_number::<u32>().unwrap(), 11);
    }

    #[test]
    fn one_multiplier() {
        // 1 multiplier
        assert_eq!("بیست و یک".words_to_number::<u32>().unwrap(), 21);
        assert_eq!("نود و نه".words_to_number::<u32>().unwrap(), 99);
        assert_eq!("دویست و هشت".words_to_number::<u32>().unwrap(), 208);

        assert_eq!("هزار و یازده".words_to_number::<u32>().unwrap(), 1011);
        assert_eq!("یک هزار و یازده".words_to_number::<u32>().unwrap(), 1011);

        assert_eq!(
            "میلیون و یازده".words_to_number::<u32>().unwrap(),
            1_000_011
        );

        assert_eq!(
            "صد و دوازده هزار و بیست و شش"
                .words_to_number::<u32>()
                .unwrap(),
            112_026
        );
    }

    #[test]
    fn two_multiplier() {
        // 2 multiplier
        assert_eq!(
            "صد و دوازده میلیون و بیست و شش هزار و پانصد و بیست و نه"
                .words_to_number::<u32>()
                .unwrap(),
            112_026_529
        );
    }

    #[test]
    fn duplicate_and_token_is_fixed() {
        // duplicate "and" is taken care of: TODO: maybe we don't want this.
        assert_eq!(
            "صد و و دوازده هزار و بیست و شش"
                .words_to_number::<u32>()
                .unwrap(),
            112_026
        );
    }

    #[test]
    fn invalid_token_fails() {
        // any gibberish fails
        assert!("صد و و کیان هزار و بیست و شش"
            .words_to_number::<u32>()
            .is_err());
    }

    #[test]
    fn boundaries() {
        // boundaries
        // assert_eq!("صفر".words_to_number::<u32>().unwrap(), 0);
        assert_eq!(
			"نهصد و نود و نه میلیارد و نهصد و نود و نه میلیون و نهصد و نود و نه هزار و نهصد و نود و نه"
				.words_to_number::<u128>()
				.unwrap(),
			999_999_999_999
		)
    }

    #[test]
    fn can_work_in_generic() {
        // generic N size must be big enough
        assert!(
			"نهصد و نود و نه میلیارد و نهصد و نود و نه میلیون و نهصد و نود و نه هزار و نهصد و نود و نه"
				.words_to_number::<u32>()
				.is_err()
		);

        // for u8
        assert_eq!("دویست و پنجاه و پنج".words_to_number::<u8>().unwrap(), 255);
        assert!("دویست و پنجاه و شش".words_to_number::<u8>().is_err());

        // for u16
        assert_eq!(
            "شصت و پنج هزار و پانصد و سی و پنج"
                .words_to_number::<u16>()
                .unwrap(),
            65535
        );
        assert!("شصت و پنج هزار و پانصد و سی و شش"
            .words_to_number::<u8>()
            .is_err());

        // for u32
        assert_eq!(
            "چهار میلیارد و دویست و نود و چهار میلیون و نهصد شصت و هفت هزار و دویست و نود و پنج"
                .words_to_number::<u32>()
                .unwrap(),
            4_294_967_295
        );
        assert!(
            "چهار میلیارد و دویست و نود و چهار میلیون و نهصد شصت و هفت هزار و دویست و نود و شش"
                .words_to_number::<u32>()
                .is_err(),
        );
    }
}
#[cfg(test)]
mod digits {
    use super::*;

    #[test]
    fn digits_en_to_fa() {
        assert_eq!("0123456789abc".digits_en_to_fa(), "۰۱۲۳۴۵۶۷۸۹abc");
    }

    #[test]
    fn digits_fa_to_en() {
        assert_eq!("۰۱۲۳۴۵۶۷۸۹abc".digits_fa_to_en(), "0123456789abc");
    }

    #[test]
    fn digits_ar_to_fa() {
        assert_eq!("٠١٢٣٤٥٦٧٨٩abc".digits_ar_to_fa(), "۰۱۲۳۴۵۶۷۸۹abc");
    }

    #[test]
    fn digits_ar_to_en() {
        assert_eq!("٠١٢٣٤٥٦٧٨٩abc".digits_ar_to_en(), "0123456789abc");
    }
}
