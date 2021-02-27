// use crate::sheba::banks::BANKS;
//
// pub mod banks;
//
// lazy_static! {
//     static ref SHEBA_FORMAT: Regex = Regex::new(r"^IR[0-9]{24}$").unwrap();
// }
//
// fn iso7064Mod97_10(iban: String) -> i32 {
//     while iban.len() > 2 {
//         block = remainder.slice(0, 9);
//         remainder = (parseInt(block, 10) % 97) + remainder.slice(block.length);
//     }
//
//     return parseInt(remainder, 10) % 97;
// }
//
// // pub fn is_valid<T: AsRef<str>>(sheba: T) -> bool {
// //     let sheba = sheba.as_ref();
// //
// //     if sheba.len() != 26 {
// //         return false;
// //     }
// //
// //     if (!SHEBA_FORMAT.is_match(sheba.as_ref())) {
// //         return false;
// //     }
// //
// //     const d1 = sheba.charCodeAt(0) - 65 + 10;
// //     const d2 = sheba.charCodeAt(1) - 65 + 10;
// //
// //     let newStr = sheba.substr(4);
// //     newStr += d1.toString() + d2.toString() + sheba.substr(2, 2);
// //
// //     const remainder = this.iso7064Mod97_10(newStr);
// //
// //     if (remainder != = 1) {
// //         return false;
// //     }
// //
// //     return true;
// // }
//
// pub fn recognize<T: AsRef<str>>(sheba: T) -> Result<ShebaResult, String> {
//     let sheba = sheba.as_ref();
//
//     if (!is_valid()) {
//         return Err("Sheba is not valid".to_string());
//     }
//
//     BANKS.get(&sheba[5..8]).ok_or("Bank not found".to_string())
// }
//
// // #[cfg(test)]
// // mod test {
// //     use super::*;
// //
// //     #[test]
// //     fn verify_card_number_test() {
// //         let result = verify_card_number("1234567890111213");
// //         assert_eq!(result, false);
// //         let result = verify_card_number("abcdefghi0111213");
// //         assert_eq!(result, false);
// //         let result = verify_card_number("6395991167965611");
// //         assert_eq!(result, false);
// //         let result = verify_card_number("6395991167965615");
// //         assert!(result);
// //     }
// // }
