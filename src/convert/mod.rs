const DIGITS: [[char; 10]; 2] = [
    ['۰', '۱', '۲', '۳', '۴', '۵', '۶', '۷', '۸', '۹'],
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
];

#[derive(Clone, Copy)]
pub enum Lang {
    Fa,
    En,
}

fn digits_convert(src: &str, from: Lang, to: Lang) -> String {
    src.chars()
        .map(|v| {
            let index = DIGITS[from as usize].iter().position(|&r| r == v);
            if index.is_some() {
                return DIGITS[to as usize][index.unwrap()];
            }
            return v;
        })
        .collect::<String>()
}

pub fn digits_tofa(src: &str) -> String {
    digits_convert(src, Lang::En, Lang::Fa)
}

pub fn digits_toen(src: &str) -> String {
    digits_convert(src, Lang::Fa, Lang::En)
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn digits_tofa_test() {
        let res = digits_tofa("1a2b3c");
        assert_eq!(res, "۱a۲b۳c");
    }

    #[test]
    pub fn digits_toen_test() {
        let res = digits_toen("a۱b۲c۳d");
        assert_eq!(res, "a1b2c3d");
    }
}
