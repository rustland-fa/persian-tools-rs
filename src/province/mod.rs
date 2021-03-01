pub mod city;

use city::*;
use phf::phf_map;
use std::fmt;
use strum::EnumString;

// in future if phf support enum as key enum must be replace with string
pub static PROVINCES: phf::Map<&'static str, Province> = phf_map! {
    "Alborz" => Province{
        prefix_phone : "026",
        farsi_name : "",
        latin_name : "",
        cities : &ALBORZ_CITIES,
    },
    "Ardabil" => Province{
        prefix_phone : "045",
        farsi_name : "",
        latin_name : "",
        cities : &ARDABIL_CITIES,
    },
    "AzerbaijanEast" => Province{
        prefix_phone : "041",
        farsi_name : "",
        latin_name : "",
        cities : &AZERBAIJAN_EAST_CITIES,
    },
    "AzerbaijanWest" => Province{
        prefix_phone : "044",
        farsi_name : "",
        latin_name : "",
        cities : &AZERBAIJAN_WEST_CITIES,
    },
    "Bushehr" => Province{
        prefix_phone : "077",
        farsi_name : "",
        latin_name : "",
        cities : &BUSHEHR_CITIES,
    },
    "ChaharMahaalAndBakhtiari" => Province{
        prefix_phone : "038",
        farsi_name : "",
        latin_name : "",
        cities : &CHAHARMAHAAL_AND_BAKHTIARI_CITIES,
    },
    "Fars" => Province{
        prefix_phone : "071",
        farsi_name : "",
        latin_name : "",
        cities : &FARS_CITIES,
    },
    "Gilan" => Province{
        prefix_phone : "013",
        farsi_name : "",
        latin_name : "",
        cities : &GILAN_CITIES,
    },
    "Golestan" => Province{
        prefix_phone : "017",
        farsi_name : "",
        latin_name : "",
        cities : &GOLESTAN_CITIES,
    },
    "Hamadan" => Province{
        prefix_phone : "081",
        farsi_name : "",
        latin_name : "",
        cities : &HAMADAN_CITIES,
    },
    "Hormozgan" => Province{
        prefix_phone : "076",
        farsi_name : "",
        latin_name : "",
        cities : &HORMOZGAN_CITIES,
    },
    "Ilam" => Province{
        prefix_phone : "084",
        farsi_name : "",
        latin_name : "",
        cities : &ILAM_CITIES,
    },
    "Isfahan" => Province{
        prefix_phone : "031",
        farsi_name : "",
        latin_name : "",
        cities : &ISFAHAN_CITIES,
    },
    "Kerman" => Province{
        prefix_phone : "034",
        farsi_name : "",
        latin_name : "",
        cities : &KERMAN_CITIES,
    },
    "Kermanshah" => Province{
        prefix_phone : "083",
        farsi_name : "",
        latin_name : "",
        cities : &KERMANSHAH_CITIES,
    },
    "KhorasanNorth" => Province{
        prefix_phone : "058",
        farsi_name : "",
        latin_name : "",
        cities : &KHORASAN_NORTH_CITIES,
    },
    "KhorasanRazavi" => Province{
        prefix_phone : "051",
        farsi_name : "",
        latin_name : "",
        cities : &KHORASAN_RAZAVI_CITIES,
    },
    "KhorasanSouth" => Province{
        prefix_phone : "056",
        farsi_name : "",
        latin_name : "",
        cities : &KHORASAN_SOUTH_CITIES,
    },
    "Khuzestan" => Province{
        prefix_phone : "061",
        farsi_name : "",
        latin_name : "",
        cities : &KHUZESTAN_CITIES,
    },
    "KohgiluyehAndBoyerAhmad" => Province{
        prefix_phone : "074",
        farsi_name : "",
        latin_name : "",
        cities : &KOHGILUYEH_ANDBOYER_AHMAD_CITIES,
    },
    "Kurdistan" => Province{
        prefix_phone : "078",
        farsi_name : "",
        latin_name : "",
        cities : &KURDISTAN_CITIES,
    },
    "Lorestan" => Province{
        prefix_phone : "066",
        farsi_name : "",
        latin_name : "",
        cities : &LORESTAN_CITIES,
    },
    "Markazi" => Province{
        prefix_phone : "086",
        farsi_name : "",
        latin_name : "",
        cities : &MARKAZI_CITIES,
    },
    "Mazandaran" => Province{
        prefix_phone : "011",
        farsi_name : "",
        latin_name : "",
        cities : &MAZANDARAN_CITIES,
    },
    "Qazvin" => Province{
        prefix_phone : "028",
        farsi_name : "",
        latin_name : "",
        cities : &QAZVIN_CITIES,
    },
    "Qom" => Province{
        prefix_phone : "025",
        farsi_name : "",
        latin_name : "",
        cities : &QOM_CITIES,
    },
    "Semnan" => Province{
        prefix_phone : "023",
        farsi_name : "",
        latin_name : "",
        cities : &SEMNAN_CITIES,
    },
    "SistanAndBaluchestan" => Province{
        prefix_phone : "054",
        farsi_name : "",
        latin_name : "",
        cities : &SISTAN_AND_BALUCHESTAN_CITIES,
    },
    "Tehran" => Province{
        prefix_phone : "021",
        farsi_name : "",
        latin_name : "",
        cities : &TEHRAN_CITIES,
    },
    "Yazd" => Province{
        prefix_phone : "035",
        farsi_name : "",
        latin_name : "",
        cities : &YAZD_CITIES,
    },
    "Zanjan" => Province{
        prefix_phone : "024",
        farsi_name : "",
        latin_name : "",
        cities : &ZANJAN_CITIES,
    },
};

#[derive(Debug, PartialEq, EnumString)]
pub enum IranProvince {
    Alborz,
    Ardabil,
    AzerbaijanEast,
    AzerbaijanWest,
    Bushehr,
    ChaharMahaalAndBakhtiari,
    Fars,
    Gilan,
    Golestan,
    Hamadan,
    Hormozgan,
    Ilam,
    Isfahan,
    Kerman,
    Kermanshah,
    KhorasanNorth,
    KhorasanRazavi,
    KhorasanSouth,
    Khuzestan,
    KohgiluyehAndBoyerAhmad,
    Kurdistan,
    Lorestan,
    Markazi,
    Mazandaran,
    Qazvin,
    Qom,
    Semnan,
    SistanAndBaluchestan,
    Tehran,
    Yazd,
    Zanjan,
}

pub struct Province {
    pub prefix_phone: &'static str,
    pub farsi_name: &'static str,
    pub latin_name: &'static str,
    pub cities: &'static phf::Map<&'static str, city::City>,
}

impl std::fmt::Display for IranProvince {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //TODO ...
        Ok(())
    }
}
