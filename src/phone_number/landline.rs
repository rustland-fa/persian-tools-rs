use std::str::FromStr;

use crate::{
    province::{IranProvince, PROVINCES},
    utils::impl_trait_for_string_types,
};

/// A trait helper to work with landline numbers.
pub trait LandlineNumber: AsRef<str> {
    /// Check if the landline number is valid.
    fn is_valid_landline_number(&self) -> bool {
        let text = self.as_ref();
        let skip = super::get_num_skip(text);

        if text.len() - skip != 10 {
            return false;
        }

        let mut chars = text.chars().skip(skip);

        if !chars.by_ref().take(2).all(|c| ('1'..='9').contains(&c)) {
            return false;
        }

        chars.all(|c| c.is_ascii_digit())
    }

    /// Get three-digit prefix of a landline number.
    fn get_prefix_landline_number(&self) -> crate::Result<String> {
        let text = self.as_ref();
        let skip = super::get_num_skip(text);

        if text.len() - skip != 10 {
            return Err("Invalid landline number".into());
        }

        text.chars()
            .skip(skip)
            .take(2)
            .try_fold(String::with_capacity(2), |mut acc, c| {
                if ('1'..='9').contains(&c) {
                    acc.push(c);
                    Ok(acc)
                } else {
                    Err("Invalid landline number".into())
                }
            })
            .map(|s| format!("0{s}"))
    }

    /// Get province of the landline number.
    fn get_province_from_landline_number(&self) -> crate::Result<Option<IranProvince>> {
        self.get_prefix_landline_number().map(|p| {
            PROVINCES.iter().find_map(|(k, v)| {
                if v.prefix_phone == p {
                    Some(IranProvince::from_str(k).unwrap())
                } else {
                    None
                }
            })
        })
    }
}

impl_trait_for_string_types!(LandlineNumber);
