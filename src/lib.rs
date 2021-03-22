pub mod banking;
pub mod date;
pub mod digit;
pub mod national_code;
pub mod number_suffix;
pub mod persian_content;
pub mod phone_number;
pub mod province;
pub mod translate;

pub type Result<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error + 'static + Send + Sync>>;
