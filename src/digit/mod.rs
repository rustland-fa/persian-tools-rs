#[derive(Clone, Copy)]
pub enum Lang {
    Fa,
    En,
    Ar,
}

/// Set of helpers to manipulate Persian (or Arabic!) digits.
trait Digit: AsRef<str> {
    const DIGITS: [[char; 10]; 3] = [
        ['۰', '۱', '۲', '۳', '۴', '۵', '۶', '۷', '۸', '۹'],
        ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
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
}

impl Digit for String {}
impl Digit for str {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn digits_en_to_fa_test() {
        assert_eq!("0123456789abc".digits_en_to_fa(), "۰۱۲۳۴۵۶۷۸۹abc");
    }

    #[test]
    fn digits_fa_to_en_test() {
        assert_eq!("۰۱۲۳۴۵۶۷۸۹abc".digits_fa_to_en(), "0123456789abc");
    }

    #[test]
    fn digits_ar_to_fa_test() {
        assert_eq!("٠١٢٣٤٥٦٧٨٩abc".digits_ar_to_fa(), "۰۱۲۳۴۵۶۷۸۹abc");
    }

    #[test]
    fn digits_ar_to_en_test() {
        assert_eq!("٠١٢٣٤٥٦٧٨٩abc".digits_ar_to_en(), "0123456789abc");
    }
}
