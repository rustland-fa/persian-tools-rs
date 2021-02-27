pub struct Bank {
    pub slug: &'static str,
    pub english_name: &'static str,
    pub persian_name: &'static str,
}

pub static BANKS: phf::Map<&'static str, Bank> = phf::phf_map! {
    "010" => {
        slug: "central",
        english_name: "Central Bank of Iran",
        persian_name: "بانک مرکزی جمهوری اسلامی ایران",
    },
    "011" => {
        slug: "sanat-o-madan",
        english_name: "Sanat O Madan Bank",
        persian_name: "بانک صنعت و معدن",
    },
    "012" => {
        slug: "mellat",
        english_name: "Mellat Bank",
        persian_name: "بانک ملت",
    },
    "013" => {
        slug: "refah",
        english_name: "Refah Bank",
        persian_name: "بانک رفاه کارگران",
    },
    "014" => {
        slug: "maskan",
        english_name: "Maskan Bank",
        persian_name: "بانک مسکن",
    },
    "015" => {
        slug: "sepah",
        english_name: "Sepah Bank",
        persian_name: "بانک سپه",
    },
    "016" => {
        slug: "keshavarzi",
        english_name: "Keshavarzi Bank",
        persian_name: "بانک کشاورزی",
    },
    "017" => {
        slug: "melli",
        english_name: "Melli Bank",
        persian_name: "بانک ملی",
    },
    "018" => {
        slug: "tejarat",
        english_name: "Tejarat Bank",
        persian_name: "بانک تجارت",
    },
    "019" => {
        slug: "saderat",
        english_name: "Saderat Bank",
        persian_name: "بانک صادرات",
    },
    "020" => {
        slug: "tosee-saderat",
        english_name: "Tosee Saderat Bank",
        persian_name: "بانک توسعه صادرات",
    },
    "021" => {
        slug: "post",
        english_name: "Post Bank",
        persian_name: "بانک پست",
    },
    "022" => {
        slug: "toose-taavon",
        english_name: "Tosee Taavon Bank",
        persian_name: "بانک توسعه تعاون",
    },
    "051" => {
        slug: "tosee",
        english_name: "Moasese Etebari Tosee",
        persian_name: "موسسه اعتباری توسعه",
    },
    "052" => {
        slug: "ghavamin",
        english_name: "Ghavamin Bank",
        persian_name: "بانک قوامین",
    },
    "053" => {
        slug: "karafarin",
        english_name: "Karafarin Bank",
        persian_name: "بانک کارآفرین",
    },
    "054" => {
        slug: "parsian",
        english_name: "Parsian Bank",
        persian_name: "بانک پارسیان",
    },
    "055" => {
        slug: "eghtesad-novin",
        english_name: "Eghtesad Novin Bank",
        persian_name: "بانک اقتصاد نوین",
    },
    "056" => {
        slug: "saman",
        english_name: "Saman Bank",
        persian_name: "بانک سامان",
    },
    "057" => {
        slug: "pasargad",
        english_name: "Pasargad Bank",
        persian_name: "بانک پاسارگاد",
    },
    "058" => {
        slug: "sarmayeh",
        english_name: "Sarmayeh Bank",
        persian_name: "بانک سرمایه",
    },
    "059" => {
        slug: "sina",
        english_name: "Sina Bank",
        persian_name: "بانک سینا",
    },
    "060" => {
        slug: "mehr-iran",
        english_name: "Mehr Iran Bank",
        persian_name: "بانک مهر ایران",
    },
    "061" => {
        slug: "shahr",
        english_name: "City Bank",
        persian_name: "بانک شهر",
    },
    "062" => {
        slug: "ayandeh",
        english_name: "Ayandeh Bank",
        persian_name: "بانک آینده",
    },
    "063" => {
        slug: "ansar",
        english_name: "Ansar Bank",
        persian_name: "بانک انصار",
    },
    "064" => {
        slug: "gardeshgari",
        english_name: "Gardeshgari Bank",
        persian_name: "بانک گردشگری",
    },
    "065" => {
        slug: "hekmat-iranian",
        english_name: "Hekmat Iranian Bank",
        persian_name: "بانک حکمت ایرانیان",
    },
    "066" => {
        slug: "dey",
        english_name: "Dey Bank",
        persian_name: "بانک دی",
    },
    "069" => {
        slug: "iran-zamin",
        english_name: "Iran Zamin Bank",
        persian_name: "بانک ایران زمین",
    },
    "070" => {
        slug: "resalat",
        english_name: "Resalat",
        persian_name: "قرض الحسنه رسالت",
    },
    "073" => {
        slug: "kosar",
        english_name: "Kosar Credit Institute",
        persian_name: "موسسه اعتباری کوثر",
    },
    "075" => {
        slug: "melal",
        english_name: "Melal Credit Institute",
        persian_name: "موسسه اعتباری ملل",
    },
    "078" => {
        slug: "middle-east",
        english_name: "Middle East Bank",
        persian_name: "بانک خاورمیانه",
    },
    "080" => {
        slug: "noor",
        english_name: "Noor Credit Institution",
        persian_name: "موسسه اعتباری نور",
    },
    "079" => {
        slug: "mehr-eqtesad",
        english_name: "Mehr Eqtesad Bank",
        persian_name: "بانک مهر اقتصاد",
    },
    "090" => {
        slug: "mehr-iran",
        english_name: "Mehr Iran Bank",
        persian_name: "بانک مهر ایران",
    },
    "095" => {
        slug: "iran-venezuela",
        english_name: "Iran and Venezuela Bank",
        persian_name: "بانک ایران و ونزوئلا",
    },
};