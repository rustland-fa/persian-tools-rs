pub mod city;

use city::*;
use phf::phf_map;
use std::fmt;

static PROVINCES: phf::Map<&'static str, Province> = phf_map! {
    "Alborz" => Province{
        prefix_phone : "",
        farsi_name : "",
        latin_name : "",
        cities : &ALBORZ_CITIES,
    },
    // TODO ...
};

#[derive(Debug, PartialEq, Eq, Hash)]
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
