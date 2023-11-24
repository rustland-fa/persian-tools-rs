use std::str::FromStr;

use crate::province::{IranProvince, PROVINCES};

pub struct LandlineNumber(u64);

impl LandlineNumber {
    /// Get 2-digit prefix of a landline number.
    pub fn get_prefix_landline_number(&self) -> u32 {
        (self.0 / 10u64.pow(8)) as u32
    }

    /// Get province of the landline number.
    pub fn get_province(&self) -> Option<IranProvince> {
        let prefix = self.get_prefix_landline_number();

        PROVINCES.into_iter().find_map(|(k, v)| {
            if v.prefix_phone == prefix {
                Some(IranProvince::from_str(k).unwrap())
            } else {
                None
            }
        })
    }
}

impl TryFrom<u64> for LandlineNumber {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let num = super::validate_num(value).ok_or("Invalid Mobile number")?;

        let two_num = num / 10u64.pow(8);
        if !matches!(
            (two_num / 10u64.pow(1), two_num % 10u64.pow(1)),
            (1..=9, 1..=9)
        ) {
            return Err("Invalid Mobile number".into());
        }

        Ok(Self(value))
    }
}

impl FromStr for LandlineNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let skip = super::get_num_skip_str(s);

        if s.len() - skip != 10 {
            return Err("String lenght doesn't match with a Landline Number".to_owned());
        }

        let chars = s.chars().skip(skip);

        if !chars.take(2).all(|c| ('1'..='9').contains(&c)) {
            return Err("Invalid Landline number".to_owned());
        }

        Ok(Self(s[skip..].parse::<u64>().map_err(|e| e.to_string())?))
    }
}
