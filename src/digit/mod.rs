const DIGITS: [[char; 10]; 3] = [
    ['۰', '۱', '۲', '۳', '۴', '۵', '۶', '۷', '۸', '۹'],
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
    ['٠', '١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩'],
];

#[derive(Clone, Copy)]
pub enum Lang {
    Fa,
    En,
    Ar,
}

/// Takes a string and which may contain some digits, and
/// replaces the source language digits with the destination
/// language digits.
pub fn digits_convert<T: AsRef<str>>(src: T, from: Lang, to: Lang) -> String {
    let src = src.as_ref();
    src.chars()
        .map(
            |v| match DIGITS[from as usize].iter().position(|&r| r == v) {
                Some(v) => DIGITS[to as usize][v],
                None => v,
            },
        )
        .collect::<_>()
}

/// Takes a string that may contain English digits, and returns
/// a string that represents the same digits but in Persian.
pub fn digits_en_to_fa<T: AsRef<str>>(src: T) -> String {
    digits_convert(src, Lang::En, Lang::Fa)
}

/// Takes a string that may contain Persian digits, and returns
/// a string that represents the same digits but in English.
pub fn digits_fa_to_en<T: AsRef<str>>(src: T) -> String {
    digits_convert(src, Lang::Fa, Lang::En)
}

/// Takes a string that may contain Arabic digits, and returns
/// a string that represents the same digits but in Persian.
pub fn digits_ar_to_fa<T: AsRef<str>>(src: T) -> String {
    digits_convert(src, Lang::Ar, Lang::Fa)
}

/// Takes a string that may contain Arabic digits, and returns
/// a string that represents the same digits but in English.
pub fn digits_ar_to_en<T: AsRef<str>>(src: T) -> String {
    digits_convert(src, Lang::Ar, Lang::En)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn digits_en_to_fa_test() {
        let res = digits_en_to_fa("0123456789abc");
        assert_eq!(res, "۰۱۲۳۴۵۶۷۸۹abc");
    }

    #[test]
    fn digits_fa_to_en_test() {
        let res = digits_fa_to_en("۰۱۲۳۴۵۶۷۸۹abc");
        assert_eq!(res, "0123456789abc");
    }

    #[test]
    fn digits_ar_to_fa_test() {
        let res = digits_ar_to_fa("٠١٢٣٤٥٦٧٨٩abc");
        assert_eq!(res, "۰۱۲۳۴۵۶۷۸۹abc");
    }

    #[test]
    fn digits_ar_to_en_test() {
        let res = digits_ar_to_en("٠١٢٣٤٥٦٧٨٩abc");
        assert_eq!(res, "0123456789abc");
    }
}
