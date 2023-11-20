use crate::utils::{create_fixed_map, FixedMap};

#[derive(Debug)]
pub struct BankCode {
    pub code: &'static str,
    pub name: &'static str,
}

pub static BANK_CODE_TABLE: FixedMap<&str, BankCode> = create_fixed_map! {
    "636214" => BankCode {
        name: "بانک آینده",
        code: "636214",
    },
    "627412" => BankCode {
        name: "بانک اقتصاد نوین",
        code: "627412",
    },
    "627381" => BankCode {
        name: "بانک انصار",
        code: "627381",
    },
    "505785" => BankCode {
        name: "بانک ایران زمین",
        code: "505785",
    },
    "622106" => BankCode {
        name: "بانک پارسیان",
        code: "622106",
    },
    "627884" => BankCode {
        name: "بانک پارسیان",
        code: "627884",
    },
    "502229" => BankCode {
        name: "بانک پاسارگاد",
        code: "502229",
    },
    "639347" => BankCode {
        name: "بانک پاسارگاد",
        code: "639347",
    },
    "627760" => BankCode {
        name: "پست بانک ایران",
        code: "627760",
    },
    "585983" => BankCode {
        name: "بانک تجارت",
        code: "585983",
    },
    "627353" => BankCode {
        name: "بانک تجارت",
        code: "627353",
    },
    "502908" => BankCode {
        name: "بانک توسعه تعاون",
        code: "502908",
    },
    "207177" => BankCode {
        name: "بانک توسعه صادرات",
        code: "207177",
    },
    "627648" => BankCode {
        name: "بانک توسعه صادرات",
        code: "627648",
    },
    "636949" => BankCode {
        name: "بانک حکمت ایرانیان",
        code: "636949",
    },
    "585949" => BankCode {
        name: "بانک خاورمیانه",
        code: "585949",
    },
    "502938" => BankCode {
        name: "بانک دی",
        code: "502938",
    },
    "504172" => BankCode {
        name: "بانک رسالت",
        code: "504172",
    },
    "589463" => BankCode {
        name: "بانک رفاه کارگران",
        code: "589463",
    },
    "621986" => BankCode {
        name: "بانک سامان",
        code: "621986",
    },
    "589210" => BankCode {
        name: "بانک سپه",
        code: "589210",
    },
    "639607" => BankCode {
        name: "بانک سرمایه",
        code: "639607",
    },
    "639346" => BankCode {
        name: "بانک سینا",
        code: "639346",
    },
    "502806" => BankCode {
        name: "بانک شهر",
        code: "502806",
    },
    "504706" => BankCode {
        name: "بانک شهر",
        code: "504706",
    },
    "603769" => BankCode {
        name: "بانک صادرات ایران",
        code: "603769",
    },
    "903769" => BankCode {
        name: "بانک صادرات ایران",
        code: "903769",
    },
    "627961" => BankCode {
        name: "بانک صنعت و معدن",
        code: "627961",
    },
    "639370" => BankCode {
        name: "بانک قرض الحسنه مهر",
        code: "639370",
    },
    "639599" => BankCode {
        name: "بانک قوامین",
        code: "639599",
    },
    "627488" => BankCode {
        name: "بانک کارآفرین",
        code: "627488",
    },
    "603770" => BankCode {
        name: "بانک کشاورزی",
        code: "603770",
    },
    "639217" => BankCode {
        name: "بانک کشاورزی",
        code: "639217",
    },
    "505416" => BankCode {
        name: "بانک گردشگری",
        code: "505416",
    },
    "505426" => BankCode {
        name: "بانک گردشگری",
        code: "505426",
    },
    "636797" => BankCode {
        name: "بانک مرکزی ایران",
        code: "636797",
    },
    "628023" => BankCode {
        name: "بانک مسکن",
        code: "628023",
    },
    "610433" => BankCode {
        name: "بانک ملت",
        code: "610433",
    },
    "991975" => BankCode {
        name: "بانک ملت",
        code: "991975",
    },
    "170019" => BankCode {
        name: "بانک ملی ایران",
        code: "170019",
    },
    "603799" => BankCode {
        name: "بانک ملی ایران",
        code: "603799",
    },
    "606373" => BankCode {
        name: "بانک مهر ایران",
        code: "606373",
    },
    "505801" => BankCode {
        name: "موسسه کوثر",
        code: "505801",
    },
    "606256" => BankCode {
        name: "موسسه اعتباری ملل",
        code: "606256",
    },
    "628157" => BankCode {
        name: "موسسه اعتباری توسعه",
        code: "628157",
    },
};

impl FixedMap<&str, BankCode> {
    pub fn get_bank_code_from_name(&self, name: &str) -> Option<&'static str> {
        self.0
            .iter()
            .find_map(|(_, bc)| (bc.name == name).then_some(bc.code))
    }

    pub fn get_bank_name_from_code(&self, code: &str) -> Option<&'static str> {
        self.get(&code).map(|v| v.name)
    }
}
