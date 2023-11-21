static DAY_SUM: [u32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
static DAY_SUM_KABISE: [u32; 12] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];

#[derive(PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord)]
pub struct JalaliDate {
    pub day: u32,
    pub year: u32,
    pub month: u32,
}

pub fn convert_gregorian_to_jalali(year: u32, month: u32, day: u32) -> crate::Result<JalaliDate> {
    if month > 12 || month < 1 {
        return Err("Month should be between 1 and 12".into());
    }

    let days_sum = if year_is_leap(year) {
        DAY_SUM_KABISE[month as usize - 1] + day
    } else {
        DAY_SUM[month as usize - 1] + day
    };

    if days_sum < 79 {
        let days_sum = days_sum + dey_jan_diff(year);
        let jalai_year = year - 622;

        if days_sum % 30 == 0 {
            return Ok(JalaliDate {
                year: jalai_year,
                day: 30,
                month: (days_sum / 30) + 9,
            });
        } else {
            return Ok(JalaliDate {
                year: jalai_year,
                day: days_sum % 30,
                month: (days_sum / 30) + 10,
            });
        }
    } else {
        let days_sum = days_sum - 79;
        let jalali_year = year - 621;

        if days_sum <= 186 {
            if days_sum % 31 == 0 {
                return Ok(JalaliDate {
                    day: 31,
                    year: jalali_year,
                    month: days_sum / 31,
                });
            } else {
                return Ok(JalaliDate {
                    day: days_sum % 31,
                    year: jalali_year,
                    month: (days_sum / 31) + 1,
                });
            }
        } else {
            let days_sum = days_sum - 186;
            if days_sum % 30 == 0 {
                return Ok(JalaliDate {
                    day: 30,
                    year: jalali_year,
                    month: (days_sum / 30) + 6,
                });
            } else {
                return Ok(JalaliDate {
                    day: days_sum % 30,
                    year: jalali_year,
                    month: (days_sum / 30) + 7,
                });
            }
        }
    }
}

// Gets the day difference between Persian month, Dey and Gregorian month January
fn dey_jan_diff(year: u32) -> u32 {
    if year_is_leap(year) {
        return 11;
    }

    return 10;
}

fn year_is_leap(gregorian_year: u32) -> bool {
    return ((gregorian_year % 100) != 0 && (gregorian_year % 4) == 0)
        || ((gregorian_year % 100) == 0 && (gregorian_year % 400) == 0);
}

static gregorian_months: [u32; 12] = [30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 28, 31];
static gregorian_month_leap: [u32; 12] = [30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29, 31];

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct GregorianDate {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

/// month range is 1..12
/// day starts from 1
pub fn convert_jalali_to_gregorian(
    year: u32,
    month: u32,
    day: u32,
) -> crate::Result<GregorianDate> {
    let mut gregorian_year = year + 621;
    let mut gregorian_day_of_month = 0;
    let mut gregorian_month = 0;
    let march_day_diff = if year_is_leap(gregorian_year) { 12 } else { 11 };
    let mut day_count = 0;

    if (1..=6).contains(&month) {
        day_count = (month - 1) * 31 + day;
    } else {
        day_count = (6 * 31) + (month - 7) * 30 + day;
    }

    if day_count < march_day_diff {
        gregorian_day_of_month = day_count + (31 - march_day_diff);
        gregorian_month = 3;
    } else {
        let mut remain_days = day_count - march_day_diff;
        let mut i = 0;

        if year_is_leap(gregorian_year + 1) {
            while remain_days > gregorian_months[i] {
                remain_days -= gregorian_month_leap[i];
                i += 1;
            }
        } else {
            while remain_days > gregorian_months[i] {
                remain_days -= gregorian_months[i];
                i += 1;
            }
        }

        gregorian_day_of_month = remain_days;

        if i > 8 {
            gregorian_month = i - 8;
            gregorian_year += 1;
        } else {
            gregorian_month = i + 4;
        }
    }

    Ok(GregorianDate {
        year: gregorian_year,
        month: gregorian_month as u32,
        day: gregorian_day_of_month,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        convert_gregorian_to_jalali, convert_jalali_to_gregorian, GregorianDate, JalaliDate,
    };

    #[test]
    fn convert_date() {
        let result = convert_gregorian_to_jalali(2022, 2, 6).unwrap();
        assert_eq!(
            result,
            JalaliDate {
                day: 17,
                month: 11,
                year: 1400
            }
        );
    }

    #[test]
    fn convert_date_2() {
        let result = convert_gregorian_to_jalali(2015, 11, 23).unwrap();
        assert_eq!(
            result,
            JalaliDate {
                day: 2,
                month: 9,
                year: 1394
            }
        );
    }

    #[test]
    fn jalali_to_gregorian_date() {
        let result = convert_jalali_to_gregorian(1402, 8, 24).unwrap();
        assert_eq!(
            result,
            GregorianDate {
                day: 15,
                month: 11,
                year: 2023
            }
        );
    }

    #[test]
    fn jalali_to_gregorian_date_2() {
        let result = convert_jalali_to_gregorian(1402, 3, 3).unwrap();
        assert_eq!(
            result,
            GregorianDate {
                day: 24,
                month: 5,
                year: 2023
            }
        );
    }
}
