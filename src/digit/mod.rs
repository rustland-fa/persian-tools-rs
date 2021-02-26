const DIGITS: [[char; 10]; 2] = [
    ['۰', '۱', '۲', '۳', '۴', '۵', '۶', '۷', '۸', '۹'],
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
];

#[derive(Clone, Copy)]
enum Lang {
    Fa,
    En,
}

fn digits_convert<T: AsRef<str>>(src: T, from: Lang, to: Lang) -> String {
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

pub fn digits_en_to_fa<T: AsRef<str>>(src: T) -> String {
    digits_convert(src, Lang::En, Lang::Fa)
}

pub fn digits_fa_to_en<T: AsRef<str>>(src: T) -> String {
    digits_convert(src, Lang::Fa, Lang::En)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn digits_en_to_fa_test() {
        let res = digits_en_to_fa("1a2b3c");
        assert_eq!(res, "۱a۲b۳c");
    }

    #[test]
    fn digits_fa_to_en_test() {
        let res = digits_fa_to_en("a۱b۲c۳d");
        assert_eq!(res, "a1b2c3d");
    }
}
