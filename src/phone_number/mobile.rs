use std::str::FromStr;
use strum::EnumString;

/// List of Iranian mobile operators.
// in future phf crate if support enums as key we must replace str with enum
pub static IRAN_MOBILE_OPERATORS: phf::Map<&str, &[u32]> = phf::phf_map! {
    "MCI" => {
        &[
            910u32, 911u32, 912u32, 913u32, 914u32, 915u32, 916u32, 917u32, 918u32, 919u32,
            990u32, 991u32, 992u32, 993u32, 994u32,
        ]
    },
    "Irancell" => {
        &[
            930u32, 933u32, 935u32, 936u32, 937u32, 938u32, 939u32, 901u32, 902u32, 903u32,
            904u32, 905u32, 941u32,
        ]
    },
    "RightTel" => {
        &[920u32, 921u32, 922u32]
    },
    "Taliya" => {
        &[932u32]
    },
    "MTCE" => {
    &[931u32]
    },
    "TeleKish" => {
        &[934u32]
    },
    "ApTel" => {
        &[99910u32, 99911u32, 99913u32]
    },
    "Azartel" => {
        &[99914u32]
    },
    "SamanTel" => {
        &[99990u32, 99999u32, 99998u32, 99997u32, 99996u32]
    },
    "LotusTel" => {
    &[9990u32]
    },
    "ShatelMobile" => {
        &[99810u32, 99811u32, 99812u32, 99814u32, 99815u32]
    },
    "ArianTel" => {
        &[9998u32]
    },
    "Anarestan" => {
        &[994u32]
    },

};

#[derive(Debug, PartialEq, Eq, Hash, EnumString)]
pub enum IranMobileOperator {
    MCI,
    MTCE,
    TeleKish,
    ApTel,
    Azartel,
    SamanTel,
    LotusTel,
    ArianTel,
    Anarestan,
    Irancell,
    RightTel,
    Taliya,
    ShatelMobile,
}

pub struct MobileNumber(u64);

impl MobileNumber {
    /// Get the operator name of the mobile number.
    pub fn get_operator_name(&self) -> Option<IranMobileOperator> {
        IRAN_MOBILE_OPERATORS
            .into_iter()
            .find_map(|(operator, nums)| {
                nums.iter()
                    .any(|n| {
                        let len = n.checked_ilog10().unwrap_or(0) + 1;
                        *n as u64 == self.0 / 10u64.pow(10 - len)
                    })
                    .then(|| IranMobileOperator::from_str(operator).unwrap())
            })
    }
}

impl TryFrom<u64> for MobileNumber {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let num = super::validate_num(value).ok_or("Invalid Mobile number")?;

        if num / 10u64.pow(9) != 9 {
            return Err("Invalid Mobile number".into());
        }

        Ok(Self(value))
    }
}

impl FromStr for MobileNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let skip = super::get_num_skip_str(s);

        if s.len() - skip != 10 {
            return Err("String lenght doesn't match with a Mobile Number".to_owned());
        }

        if !s[skip..].starts_with('9') {
            return Err("Invalid number".to_owned());
        }

        Ok(Self(s[skip..].parse::<u64>().map_err(|e| e.to_string())?))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn is_valid_mobile_number_test() {
        assert!(MobileNumber::from_str("09398254166").is_ok());
        assert!(MobileNumber::try_from(9398254166).is_ok());
        assert!(MobileNumber::from_str("+989398254166").is_ok());
        assert!(MobileNumber::try_from(989398254166).is_ok());
        assert!(MobileNumber::from_str("+98939825416621121121122133313").is_err());
    }

    #[test]
    fn get_operator_name_from_mobile_number_test() {
        assert_eq!(
            MobileNumber::from_str("09324341133")
                .unwrap()
                .get_operator_name()
                .unwrap(),
            IranMobileOperator::Taliya
        );
        assert_eq!(
            MobileNumber::from_str("+989324341133")
                .unwrap()
                .get_operator_name()
                .unwrap(),
            IranMobileOperator::Taliya
        );
        assert_eq!(
            MobileNumber::from_str("+989134341133")
                .unwrap()
                .get_operator_name()
                .unwrap(),
            IranMobileOperator::MCI
        );
        assert_eq!(
            MobileNumber::from_str("+989999048230")
                .unwrap()
                .get_operator_name()
                .unwrap(),
            IranMobileOperator::SamanTel
        );
        assert!(MobileNumber::from_str("+98999999999").is_err());
    }
}
