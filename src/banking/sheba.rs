use crate::impl_trait_for_string_types;

#[derive(Debug)]
pub struct ShebaInfo {
    nickname: &'static str,
    name: &'static str,
    persian_name: &'static str,
    code: &'static str,
    account_available: bool,
}
pub trait ShebaNumber: AsRef<str> {
    fn is_valid_sheba_code(&self) -> bool {
        false
    }

    fn sheba_code_info(&self) -> Option<ShebaInfo> {
        None
    }
}

pub static SHEBA_CODE: phf::Map<&'static str, ShebaInfo> = phf::phf_map! {
    "010" => ShebaInfo{
        nickname: "central-bank",
        name: "Central Bank of Iran",
        persian_name: "بانک مرکزی جمهوری اسلامی ایران",
        code: "010",
        account_available: false,
    },
    "011" => ShebaInfo{
        nickname: "sanat-o-madan",
        name: "Sanat O Madan Bank",
        persian_name: "بانک صنعت و معدن",
        code: "011",
        account_available: false,
    },
    "012" => ShebaInfo{
        nickname: "mellat",
        name: "Mellat Bank",
        persian_name: "بانک ملت",
        code: "012",
        account_available: false,
    },
    "013" => ShebaInfo{
        nickname: "refah",
        name: "Refah Bank",
        persian_name: "بانک رفاه کارگران",
        code: "013",
        account_available: false,
    },
    "014" => ShebaInfo{
        nickname: "maskan",
        name: "Maskan Bank",
        persian_name: "بانک مسکن",
        code: "014",
        account_available: false,
    },
    "015" => ShebaInfo{
        nickname: "sepah",
        name: "Sepah Bank",
        persian_name: "بانک سپه",
        code: "015",
        account_available: false,
    },
    "016" => ShebaInfo{
        nickname: "keshavarzi",
        name: "Keshavarzi",
        persian_name: "بانک کشاورزی",
        code: "016",
        account_available: false,
    },
    "017" => ShebaInfo{
        nickname: "melli",
        name: "Melli",
        persian_name: "بانک ملی ایران",
        code: "017",
        account_available: false,
    },
    "018" => ShebaInfo{
        nickname: "tejarat",
        name: "Tejarat Bank",
        persian_name: "بانک تجارت",
        code: "018",
        account_available: false,
    },
    "019" => ShebaInfo{
        nickname: "saderat",
        name: "Saderat Bank",
        persian_name: "بانک صادرات ایران",
        code: "019",
        account_available: false,
    },
    "020" => ShebaInfo{
        nickname: "tosee-saderat",
        name: "Tose Saderat Bank",
        persian_name: "بانک توسعه صادرات",
        code: "020",
        account_available: false,
    },
    "021" => ShebaInfo{
        nickname: "post",
        name: "Post Bank",
        persian_name: "پست بانک ایران",
        code: "021",
        account_available: false,
    },
    "022" => ShebaInfo{
        nickname: "toose-taavon",
        name: "Tosee Taavon Bank",
        persian_name: "بانک توسعه تعاون",
        code: "022",
        account_available: false,
    },
    "051" => ShebaInfo{
        nickname: "tosee",
        name: "Tosee Bank",
        persian_name: "موسسه اعتباری توسعه",
        code: "051",
        account_available: false,
    },
    "052" => ShebaInfo{
        nickname: "ghavamin",
        name: "Ghavamin Bank",
        persian_name: "بانک قوامین",
        code: "052",
        account_available: false,
    },
    "053" => ShebaInfo{
        nickname: "karafarin",
        name: "Karafarin Bank",
        persian_name: "بانک کارآفرین",
        code: "053",
        account_available: false,
    },
    "054" => ShebaInfo{
        nickname: "parsian",
        name: "Parsian Bank",
        persian_name: "بانک پارسیان",
        code: "054",
        account_available: true,
    },
    "055" => ShebaInfo{
        nickname: "eghtesad-novin",
        name: "Eghtesad Novin Bank",
        persian_name: "بانک اقتصاد نوین",
        code: "055",
        account_available: false,
    },
    "056" => ShebaInfo{
        nickname: "saman",
        name: "Saman Bank",
        persian_name: "بانک سامان",
        code: "056",
        account_available: false,
    },
    "057" => ShebaInfo{
        nickname: "pasargad",
        name: "Pasargad Bank",
        persian_name: "بانک پاسارگاد",
        code: "057",
        account_available: true,
    },
    "058" => ShebaInfo{
        nickname: "sarmayeh",
        name: "Sarmayeh Bank",
        persian_name: "بانک سرمایه",
        code: "058",
        account_available: false,
    },
    "059" => ShebaInfo{
        nickname: "sina",
        name: "Sina Bank",
        persian_name: "بانک سینا",
        code: "059",
        account_available: false,
    },
    "060" => ShebaInfo{
        nickname: "mehr-iran",
        name: "Mehr Iran Bank",
        persian_name: "بانک مهر ایران",
        code: "060",
        account_available: false,
    },
    "" => ShebaInfo{
        nickname: "shahr",
        name: "City Bank",
        persian_name: "بانک شهر",
        code: "061",
        account_available: true,
    },
    "062" => ShebaInfo{
        nickname: "ayandeh",
        name: "Ayandeh Bank",
        persian_name: "بانک آینده",
        code: "062",
        account_available: false,
    },
    "063" => ShebaInfo{
        nickname: "ansar",
        name: "Ansar Bank",
        persian_name: "بانک انصار",
        code: "063",
        account_available: false,
    },
    "064" => ShebaInfo{
        nickname: "gardeshgari",
        name: "Gardeshgari Bank",
        persian_name: "بانک گردشگری",
        code: "064",
        account_available: false,
    },
    "065" => ShebaInfo{
        nickname: "hekmat-iranian",
        name: "Hekmat Iranian Bank",
        persian_name: "بانک حکمت ایرانیان",
        code: "065",
        account_available: false,
    },
    "066" => ShebaInfo{
        nickname: "dey",
        name: "Dey Bank",
        persian_name: "بانک دی",
        code: "066",
        account_available: false,
    },
    "069" => ShebaInfo{
        nickname: "iran-zamin",
        name: "Iran Zamin Bank",
        persian_name: "بانک ایران زمین",
        code: "069",
        account_available: false,
    },
    "070" => ShebaInfo{
        nickname: "resalat",
        name: "Resalat Bank",
        persian_name: "بانک قرض الحسنه رسالت",
        code: "070",
        account_available: false,
    },
    "073" => ShebaInfo{
        nickname: "kosar",
        name: "Kosar Credit Institute",
        persian_name: "موسسه اعتباری کوثر",
        code: "073",
        account_available: false,
    },
    "075" => ShebaInfo{
        nickname: "melal",
        name: "Melal Credit Institute",
        persian_name: "موسسه اعتباری ملل",
        code: "075",
        account_available: false,
    },
    "078" => ShebaInfo{
        nickname: "middle-east-bank",
        name: "Middle East Bank",
        persian_name: "بانک خاورمیانه",
        code: "078",
        account_available: false,
    },
    "080" => ShebaInfo{
        nickname: "noor-bank",
        name: "Noor Credit Institution",
        persian_name: "موسسه اعتباری نور",
        code: "080",
        account_available: false,
    },
    "079" => ShebaInfo{
        nickname: "mehr-eqtesad",
        name: "Mehr Eqtesad Bank",
        persian_name: "بانک مهر اقتصاد",
        code: "079",
        account_available: false,
    },
    "090" => ShebaInfo{
        nickname: "mehr-iran",
        name: "Mehr Iran Bank",
        persian_name: "بانک مهر ایران",
        code: "090",
        account_available: false,
    },
    "095" => ShebaInfo{
        nickname: "iran-venezuela",
        name: "Iran and Venezuela Bank",
        persian_name: "بانک ایران و ونزوئلا",
        code: "095",
        account_available: false,
    },
};

impl_trait_for_string_types!(ShebaNumber);