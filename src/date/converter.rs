//TODO
pub fn convert_gregorian_to_jalali(year: u32, month: u32, day: u32) {
    let ptime = ptime::from_gregorian_date(g_year, g_month, g_day);
}
//TODO
pub fn convert_jalali_to_gregorian(year: u32, month: u32, day: u32) {
    let gtime = ptime::from_persian_date(p_year, p_month, p_day);
}
