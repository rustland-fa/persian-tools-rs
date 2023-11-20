use strum::{Display, EnumString};

use crate::utils::{create_fixed_map, FixedMap};

#[derive(Debug)]
pub struct City {
    pub farsi_name: &'static str,
    pub latin_name: &'static str,
}

pub static ALBORZ_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Alborz {
    // TODO add all cities
}

pub static ARDABIL_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Ardabil {
    // TODO add all cities
}

pub static AZERBAIJAN_EAST_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum AzerbaijanEast {
    // TODO add all cities
}

pub static AZERBAIJAN_WEST_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum AzerbaijanWest {
    // TODO add all cities
}

pub static BUSHEHR_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Bushehr {
    // TODO add all cities
}

pub static CHAHARMAHAAL_AND_BAKHTIARI_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum ChaharMahaalAndBakhtiari {
    // TODO add all cities
}

pub static FARS_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Fars {
    // TODO add all cities
}

pub static GILAN_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "Rasht" => City{
        farsi_name : "رشت",
        latin_name : "Rasht",
    },
    "Langerood" => City{
        farsi_name : "لنگرود",
        latin_name : "Langerood",
    },
    "Lahijan" => City{
        farsi_name : "لاهیجان",
        latin_name : "Lahijan",
    },
    "Astara" => City{
        farsi_name : "آستارا",
        latin_name : "Astara",
    },
    "Shaft" => City{
        farsi_name : "شفت",
        latin_name : "Shaft",
    },
    "Masal" => City{
        farsi_name : "ماسال",
        latin_name : "Masal",
    },
    "Siahkal" => City{
        farsi_name : "سیاهکل",
        latin_name : "Siahkal",
    },
    "BandarAnzali" => City{
        farsi_name : "بندرانزلی",
        latin_name : "BandarAnzali",
    },
    "Talesh" => City{
        farsi_name : "تالش",
        latin_name : "Talesh",
    },
    "Rudsar" => City{
        farsi_name : "رودسر",
        latin_name : "Rudsar",
    },
    "Rudbar" => City{
        farsi_name : "رودبار",
        latin_name : "Rudbar",
    },
    "Fouman" => City{
        farsi_name : "فومن",
        latin_name : "Fouman",
    },
    "Amlash" => City{
        farsi_name : "املش",
        latin_name : "Amlash",
    },
    "Rezvanshahr" => City{
        farsi_name : "رضوانشهر",
        latin_name : "Rezvanshahr",
    },
    "SomeSara" => City{
        farsi_name : "صومعه‌ سرا",
        latin_name : "SomeSara",
    },
    "AstanehAshrafieh" => City{
        farsi_name : "آستانه اشرفیه",
        latin_name : "AstanehAshrafieh",
    },
    "Khomam" => City{
        farsi_name : "خمام",
        latin_name : "Khomam",
    },
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Gilan {
    Rasht,
    Langerood,
    Lahijan,
    Astara,
    Shaft,
    Masal,
    Siahkal,
    BandarAnzali,
    Talesh,
    Rudsar,
    Rudbar,
    Fouman,
    Amlash,
    Rezvanshahr,
    SomeSara,
    AstanehAshrafieh,
    Khomam,
}

pub static GOLESTAN_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Golestan {
    // TODO add all cities
}

pub static HAMADAN_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Hamadan {
    // TODO add all cities
}

pub static HORMOZGAN_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Hormozgan {
    // TODO add all cities
}

pub static ILAM_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Ilam {
    // TODO add all cities
}

pub static ISFAHAN_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Isfahan {
    // TODO add all cities
}

pub static KERMAN_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Kerman {
    // TODO add all cities
}

pub static KERMANSHAH_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Kermanshah {
    // TODO add all cities
}

pub static KHORASAN_NORTH_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum KhorasanNorth {
    // TODO add all cities
}

pub static KHORASAN_RAZAVI_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum KhorasanRazavi {
    // TODO add all cities
}

pub static KHORASAN_SOUTH_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum KhorasanSouth {
    // TODO add all cities
}

pub static KHUZESTAN_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Khuzestan {
    // TODO add all cities
}

pub static KOHGILUYEH_ANDBOYER_AHMAD_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum KohgiluyehAndBoyerAhmad {
    // TODO add all cities
}

pub static KURDISTAN_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Kurdistan {
    // TODO add all cities
}

pub static LORESTAN_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Lorestan {
    // TODO add all cities
}

pub static MARKAZI_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Markazi {
    // TODO add all cities
}

pub static MAZANDARAN_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Mazandaran {
    // TODO add all cities
}

pub static QAZVIN_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Qazvin {
    // TODO add all cities
}

pub static QOM_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Qom {
    // TODO add all cities
}

pub static SEMNAN_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Semnan {
    // TODO add all cities
}

pub static SISTAN_AND_BALUCHESTAN_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "Khash" => City{
        farsi_name : "خاش",
        latin_name : "Khash",
    },
    "Zahedan" => City{
        farsi_name : "زاهدان",
        latin_name : "Zahedan",
    },
    "Zabol" => City{
        farsi_name : "زابل",
        latin_name : "Zabol",
    },
    "Iranshahr" => City{
        farsi_name : "ایرانشهر",
        latin_name : "Iranshahr",
    },
    "Chabahar" => City{
        farsi_name : "چابهار",
        latin_name : "Chabahar",
    },
    "Saravan" => City{
        farsi_name : "سراوان",
        latin_name : "Saravan",
    },
    "Nikshahr" => City{
        farsi_name : "نیکشهر",
        latin_name : "Nikshahr",
    },
    "Rask" => City{
        farsi_name : "راسک",
        latin_name : "Rask",
    },
    "Konarak" => City{
        farsi_name : "کنارک",
        latin_name : "Konarak",
    },
    "Zahak" => City{
        farsi_name : "زهک",
        latin_name : "Zahak",
    },
    "Delgan" => City{
        farsi_name : "دلگان",
        latin_name : "Delgan",
    },
    "SibVaSoran" => City{
        farsi_name : "سیب و سوران",
        latin_name : "SibVaSoran",
    },
    "Hirmand" => City{
        farsi_name : "هیرمند",
        latin_name : "Hirmand",
    },
    "Mehrestan" => City{
        farsi_name : "مهرستان",
        latin_name : "Mehrestan",
    },
    "Mirjaveh" => City{
        farsi_name : "میرجاوه",
        latin_name : "Mirjaveh",
    },
    "Ghasreghand" => City{
        farsi_name : "قصرقند",
        latin_name : "Ghasreghand",
    },
    "Nimroz" => City{
        farsi_name : "نیمروز",
        latin_name : "Nimroz",
    },
    "Haamon" => City{
        farsi_name : "هامون",
        latin_name : "Haamon",
    },
    "Fenoj" => City{
        farsi_name : "فنوج",
        latin_name : "Fenoj",
    },
    "Bempor" => City{
        farsi_name : "بمپور",
        latin_name : "Bempor",
    },
    "Taftan" => City{
        farsi_name : "تفتان",
        latin_name : "Taftan",
    },
    "Dashtyari" => City{
        farsi_name : "دشتیاری",
        latin_name : "Dashtyari",
    },
    "Sarbaz" => City{
        farsi_name : "سرباز",
        latin_name : "Sarbaz",
    },
    "Golshan" => City{
        farsi_name : "گلشن",
        latin_name : "Golshan",
    },
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum SistanAndBaluchestan {
    Khash,
    Zahedan,
    Zabol,
    Iranshahr,
    Chabahar,
    Saravan,
    Nikshahr,
    Rask,
    Konarak,
    Zahak,
    Delgan,
    SibVaSoran,
    Hirmand,
    Mehrestan,
    Mirjaveh,
    Ghasreghand,
    Nimroz,
    Haamon,
    Fenoj,
    Bempor,
    Taftan,
    Dashtyari,
    Sarbaz,
    Golshan,
}

pub static TEHRAN_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "EslamShahr"=> City{
        farsi_name : "اسلامشهر",
        latin_name : "EslamShahr",
    },
    "Baharestan"=> City{
        farsi_name : "بهارستان",
        latin_name : "Baharestan",
    },
    "Pakdasht"=> City{
        farsi_name : "پاکدشت",
        latin_name : "Pakdasht",
    },
    "Pardis"=> City{
        farsi_name : "پردیس",
        latin_name : "Pardis",
    },
    "Pishva"=> City{
        farsi_name : "پیشوا",
        latin_name : "Pishva",
    },
    "Tehran"=> City{
        farsi_name : "تهران",
        latin_name : "Tehran",
    },
    "Damavand"=> City{
        farsi_name : "دماوند",
        latin_name : "Damavand",
    },
    "Robatkarim"=> City{
        farsi_name : "رباط‌کریم",
        latin_name : "Robatkarim",
    },
    "Rey"=> City{
        farsi_name : "ری",
        latin_name : "Rey",
    },
    "Shemiranat"=> City{
        farsi_name : "شمیرانات",
        latin_name : "Shemiranat",
    },
    "Shahryar"=> City{
        farsi_name : "شهریار",
        latin_name : "Shahryar",
    },
    "Ghods"=> City{
        farsi_name : "قدس",
        latin_name : "Ghods",
    },
    "Gharchak"=> City{
        farsi_name : "قرچک",
        latin_name : "Gharchak",
    },
    "Firozkoh"=> City{
        farsi_name : "فیروزکوه",
        latin_name : "Firozkoh",
    },
    "Malard"=> City{
        farsi_name : "ملارد",
        latin_name : "Malard",
    },
    "Varamin"=> City{
        farsi_name : "ورامین",
        latin_name : "Varamin",
    },
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Tehran {
    EslamShahr,
    Baharestan,
    Pakdasht,
    Pardis,
    Pishva,
    Tehran,
    Damavand,
    Robatkarim,
    Rey,
    Shemiranat,
    Shahryar,
    Ghods,
    Gharchak,
    Firozkoh,
    Malard,
    Varamin,
}

pub static YAZD_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "Yasd" => City{
        farsi_name : "یزد",
        latin_name : "Yasd",
    },
    "Maybod" => City{
        farsi_name : "میبد",
        latin_name : "Maybod",
    },
    "Ardakan" =>  City{
        farsi_name : "اردکان",
        latin_name : "Ardakan",
    },
    "Mehriz" =>  City{
        farsi_name : "مهریز",
        latin_name : "Mehriz",
    },
    "Abarkoh" =>  City{
        farsi_name : "ابرکوه",
        latin_name : "Abarkoh",
    },
    "Bafgh" =>  City{
        farsi_name : "بافق",
        latin_name : "Bafgh",
    },
    "Taft" =>  City{
        farsi_name : "تفت",
        latin_name : "Taft",
    },
    "Khatam" =>  City{
        farsi_name : "خاتم",
        latin_name : "Khatam",
    },
    "Ashkezar" =>  City{
        farsi_name : "اشکذر",
        latin_name : "Ashkezar",
    },
    "Bahabad" =>  City{
        farsi_name : "بهاباد",
        latin_name : "Bahabad",
    },
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Yazd {
    Yasd,
    Maybod,
    Ardakan,
    Mehriz,
    Abarkoh,
    Bafgh,
    Taft,
    Khatam,
    Ashkezar,
    Bahabad,
}

pub static ZANJAN_CITIES: FixedMap<&str, City> = create_fixed_map! {
    "Zanjan" => City{
        farsi_name : "زنجان",
        latin_name : "Zanjan",
    },
    "Abhar" => City{
        farsi_name : "ابهر",
        latin_name : "Abhar",
    },
    "Khodabandeh" => City{
        farsi_name : "خدابنده",
        latin_name : "Khodabandeh",
    },
    "KhoramDare" => City{
        farsi_name : "خرمدره",
        latin_name : "Khoramdare",
    },
    "Taram" => City{
        farsi_name : "طارم",
        latin_name : "Taram",
    },
    "MahNeshan" => City{
        farsi_name : "ماهنشان",
        latin_name : "Mahneshan",
    },
    "EijRod" => City{
        farsi_name : "ایجرود",
        latin_name : "Eijrod",
    },
    "Soltanie" => City{
        farsi_name : "سلطانیه",
        latin_name : "Soltanie",
    },
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Zanjan {
    Zanjan,
    Abhar,
    Khodabandeh,
    Khoramdare,
    Taram,
    Mahneshan,
    Eijrod,
    Soltanie,
}
