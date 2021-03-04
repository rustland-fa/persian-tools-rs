use strum::{Display, EnumString};
#[derive(Debug)]
pub struct City {
    pub farsi_name: &'static str,
    pub latin_name: &'static str,
}

pub static ALBORZ_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static ARDABIL_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static AZERBAIJAN_EAST_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static AZERBAIJAN_WEST_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static BUSHEHR_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static CHAHARMAHAAL_AND_BAKHTIARI_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static FARS_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static GILAN_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Gilan {
    // TODO add all cities
}

pub static GOLESTAN_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static HAMADAN_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static HORMOZGAN_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static ILAM_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static ISFAHAN_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static KERMAN_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static KERMANSHAH_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static KHORASAN_NORTH_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static KHORASAN_RAZAVI_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static KHORASAN_SOUTH_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static KHUZESTAN_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static KOHGILUYEH_ANDBOYER_AHMAD_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static KURDISTAN_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static LORESTAN_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static MARKAZI_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static MAZANDARAN_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static QAZVIN_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static QOM_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static SEMNAN_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static SISTAN_AND_BALUCHESTAN_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum SistanAndBaluchestan {
    // TODO add all cities
}

pub static TEHRAN_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
    "" => City{
        farsi_name : "",
        latin_name : "",
    },
    // TODO ...
};
#[derive(Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Tehran {
    // TODO add all cities
}

pub static YAZD_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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

pub static ZANJAN_CITIES: phf::Map<&'static str, City> = phf::phf_map! {
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
