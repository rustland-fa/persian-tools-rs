use crate::utils::{create_fixed_map, FixedMap};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShebaAccountNumber {
    pub normal: String,
    pub formatted: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShebaInfo {
    pub code: &'static str,
    pub nickname: &'static str,
    pub name: &'static str,
    pub persian_name: &'static str,
    pub account_number: Option<ShebaAccountNumber>,
    pub process: Option<fn(&mut ShebaInfo, &str)>,
}

impl ShebaInfo {
    /// Process the sheba if needed and return a newly created [`ShebaInfo`].
    pub fn process(&self, sheba_code: &str) -> ShebaInfo {
        let mut sheba_clone = self.clone();
        if let Some(process) = self.process {
            process(&mut sheba_clone, sheba_code);
        }
        sheba_clone
    }
}

pub static SHEBA_CODE_TABLE: FixedMap<&str, ShebaInfo> = create_fixed_map! {
    "010" => ShebaInfo{
        code: "010",
        nickname: "central-bank",
        name: "Central Bank of Iran",
        persian_name: "بانک مرکزی جمهوری اسلامی ایران",
        account_number: None,
        process: None,
    },
    "011" => ShebaInfo{
        code: "011",
        nickname: "sanat-o-madan",
        name: "Sanat O Madan Bank",
        persian_name: "بانک صنعت و معدن",
        account_number: None,
        process: None,
    },
    "012" => ShebaInfo{
        code: "012",
        nickname: "mellat",
        name: "Mellat Bank",
        persian_name: "بانک ملت",
        account_number: None,
        process: None,
    },
    "013" => ShebaInfo{
        code: "013",
        nickname: "refah",
        name: "Refah Bank",
        persian_name: "بانک رفاه کارگران",
        account_number: None,
        process: None,
    },
    "014" => ShebaInfo{
        code: "014",
        nickname: "maskan",
        name: "Maskan Bank",
        persian_name: "بانک مسکن",
        account_number: None,
        process: None,
    },
    "015" => ShebaInfo{
        code: "015",
        nickname: "sepah",
        name: "Sepah Bank",
        persian_name: "بانک سپه",
        account_number: None,
        process: None,
    },
    "016" => ShebaInfo{
        code: "016",
        nickname: "keshavarzi",
        name: "Keshavarzi",
        persian_name: "بانک کشاورزی",
        account_number: None,
        process: None,
    },
    "017" => ShebaInfo{
        code: "017",
        nickname: "melli",
        name: "Melli",
        persian_name: "بانک ملی ایران",
        account_number: None,
        process: None,
    },
    "018" => ShebaInfo{
        code: "018",
        nickname: "tejarat",
        name: "Tejarat Bank",
        persian_name: "بانک تجارت",
        account_number: None,
        process: None,
    },
    "019" => ShebaInfo{
        code: "019",
        nickname: "saderat",
        name: "Saderat Bank",
        persian_name: "بانک صادرات ایران",
        account_number: None,
        process: None,
    },
    "020" => ShebaInfo{
        code: "020",
        nickname: "tosee-saderat",
        name: "Tose Saderat Bank",
        persian_name: "بانک توسعه صادرات",
        account_number: None,
        process: None,
    },
    "021" => ShebaInfo{
        code: "021",
        nickname: "post",
        name: "Post Bank",
        persian_name: "پست بانک ایران",
        account_number: None,
        process: None,
    },
    "022" => ShebaInfo{
        code: "022",
        nickname: "toose-taavon",
        name: "Tosee Taavon Bank",
        persian_name: "بانک توسعه تعاون",
        account_number: None,
        process: None,
    },
    "051" => ShebaInfo{
        code: "051",
        nickname: "tosee",
        name: "Tosee Bank",
        persian_name: "موسسه اعتباری توسعه",
        account_number: None,
        process: None,
    },
    "052" => ShebaInfo{
        code: "052",
        nickname: "ghavamin",
        name: "Ghavamin Bank",
        persian_name: "بانک قوامین",
        account_number: None,
        process: None,
    },
    "053" => ShebaInfo{
        code: "053",
        nickname: "karafarin",
        name: "Karafarin Bank",
        persian_name: "بانک کارآفرین",
        account_number: None,
        process: None,
    },
    "054" => ShebaInfo{
        code: "054",
        nickname: "parsian",
        name: "Parsian Bank",
        persian_name: "بانک پارسیان",
        account_number: None,
        process: Some(process_parsian),
    },
    "055" => ShebaInfo{
        code: "055",
        nickname: "eghtesad-novin",
        name: "Eghtesad Novin Bank",
        persian_name: "بانک اقتصاد نوین",
        account_number: None,
        process: None,
    },
    "056" => ShebaInfo{
        code: "056",
        nickname: "saman",
        name: "Saman Bank",
        persian_name: "بانک سامان",
        account_number: None,
        process: None,
    },
    "057" => ShebaInfo{
        code: "057",
        nickname: "pasargad",
        name: "Pasargad Bank",
        persian_name: "بانک پاسارگاد",
        account_number: None,
        process: Some(process_pasargad),
    },
    "058" => ShebaInfo{
        code: "058",
        nickname: "sarmayeh",
        name: "Sarmayeh Bank",
        persian_name: "بانک سرمایه",
        account_number: None,
        process: None,
    },
    "059" => ShebaInfo{
        code: "059",
        nickname: "sina",
        name: "Sina Bank",
        persian_name: "بانک سینا",
        account_number: None,
        process: None,
    },
    "060" => ShebaInfo{
        code: "060",
        nickname: "mehr-iran",
        name: "Mehr Iran Bank",
        persian_name: "بانک مهر ایران",
        account_number: None,
        process: None,
    },
    "061" => ShebaInfo{
        code: "061",
        nickname: "shahr",
        name: "City Bank",
        persian_name: "بانک شهر",
        account_number: None,
        process: Some(process_shahr),
    },
    "062" => ShebaInfo{
        code: "062",
        nickname: "ayandeh",
        name: "Ayandeh Bank",
        persian_name: "بانک آینده",
        account_number: None,
        process: None,
    },
    "063" => ShebaInfo{
        code: "063",
        nickname: "ansar",
        name: "Ansar Bank",
        persian_name: "بانک انصار",
        account_number: None,
        process: None,
    },
    "064" => ShebaInfo{
        code: "064",
        nickname: "gardeshgari",
        name: "Gardeshgari Bank",
        persian_name: "بانک گردشگری",
        account_number: None,
        process: None,
    },
    "065" => ShebaInfo{
        code: "065",
        nickname: "hekmat-iranian",
        name: "Hekmat Iranian Bank",
        persian_name: "بانک حکمت ایرانیان",
        account_number: None,
        process: None,
    },
    "066" => ShebaInfo{
        code: "066",
        nickname: "dey",
        name: "Dey Bank",
        persian_name: "بانک دی",
        account_number: None,
        process: None,
    },
    "069" => ShebaInfo{
        code: "069",
        nickname: "iran-zamin",
        name: "Iran Zamin Bank",
        persian_name: "بانک ایران زمین",
        account_number: None,
        process: None,
    },
    "070" => ShebaInfo{
        code: "070",
        nickname: "resalat",
        name: "Resalat Bank",
        persian_name: "بانک قرض الحسنه رسالت",
        account_number: None,
        process: None,
    },
    "073" => ShebaInfo{
        code: "073",
        nickname: "kosar",
        name: "Kosar Credit Institute",
        persian_name: "موسسه اعتباری کوثر",
        account_number: None,
        process: None,
    },
    "075" => ShebaInfo{
        code: "075",
        nickname: "melal",
        name: "Melal Credit Institute",
        persian_name: "موسسه اعتباری ملل",
        account_number: None,
        process: None,
    },
    "078" => ShebaInfo{
        code: "078",
        nickname: "middle-east-bank",
        name: "Middle East Bank",
        persian_name: "بانک خاورمیانه",
        account_number: None,
        process: None,
    },
    "080" => ShebaInfo{
        code: "080",
        nickname: "noor-bank",
        name: "Noor Credit Institution",
        persian_name: "موسسه اعتباری نور",
        account_number: None,
        process: None,
    },
    "079" => ShebaInfo{
        code: "079",
        nickname: "mehr-eqtesad",
        name: "Mehr Eqtesad Bank",
        persian_name: "بانک مهر اقتصاد",
        account_number: None,
        process: None,
    },
    "090" => ShebaInfo{
        code: "090",
        nickname: "mehr-iran",
        name: "Mehr Iran Bank",
        persian_name: "بانک مهر ایران",
        account_number: None,
        process: None,
    },
    "095" => ShebaInfo{
        code: "095",
        nickname: "iran-venezuela",
        name: "Iran and Venezuela Bank",
        persian_name: "بانک ایران و ونزوئلا",
        account_number: None,
        process: None,
    },
};

pub(super) fn process_parsian(sheba: &mut ShebaInfo, sheba_code: &str) {
    let substr = &sheba_code[14..];
    sheba.account_number = Some(ShebaAccountNumber {
        normal: substr.to_owned(),
        formatted: format!("0{}-0{}-{}", &substr[0..2], &substr[2..9], &substr[9..12]),
    });
}

pub(super) fn process_pasargad(sheba: &mut ShebaInfo, sheba_code: &str) {
    let mut idx = 7;
    for ch in sheba_code[7..].chars() {
        if ch != '0' {
            break;
        }
        idx += 1;
    }
    let substr = &sheba_code[idx..sheba_code.len() - 2];
    sheba.account_number = Some(ShebaAccountNumber {
        normal: substr.to_owned(),
        formatted: format!(
            "{}-{}-{}-{}",
            &substr[0..3],
            &substr[3..6],
            &substr[6..14],
            &substr[14..15]
        ),
    });
}

pub(super) fn process_shahr(sheba: &mut ShebaInfo, sheba_code: &str) {
    let mut idx = 7;
    for ch in sheba_code[7..].chars() {
        if ch != '0' {
            break;
        }
        idx += 1;
    }
    let substr = &sheba_code[idx..];
    sheba.account_number = Some(ShebaAccountNumber {
        normal: substr.to_owned(),
        formatted: substr.to_owned(),
    });
}
