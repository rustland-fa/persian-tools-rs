use reqwest::header::USER_AGENT;
use std::str::FromStr;
use strum::{Display, EnumString};

use crate::utils::impl_trait_for_string_types;

/// Languages that can used for input and output of the [`translate`] function.
#[derive(Debug, Clone, PartialEq, Copy, Hash, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Language {
    #[strum(serialize = "english", serialize = "en", serialize = "انگلیسی")]
    English,
    #[strum(serialize = "farsi", serialize = "fa", serialize = "فارسی")]
    Farsi,
    #[strum(serialize = "arabic", serialize = "ar", serialize = "عربی")]
    Arabic,
    #[strum(serialize = "chinese", serialize = "zh", serialize = "چینی")]
    Chinese,
    #[strum(serialize = "french", serialize = "fr", serialize = "فرانسوی")]
    French,
    #[strum(serialize = "german", serialize = "de", serialize = "آلمانی")]
    German,
    #[strum(serialize = "italian", serialize = "it", serialize = "ایتالیایی")]
    Italian,
    #[strum(serialize = "japanese", serialize = "ja", serialize = "ژاپنی")]
    Japanese,
    #[strum(serialize = "portuguese", serialize = "pt", serialize = "پرتغالی")]
    Portuguese,
    #[strum(serialize = "russian", serialize = "ru", serialize = "روسی")]
    Russian,
    #[strum(serialize = "spanish", serialize = "es", serialize = "اسپانیایی")]
    Spanish,
}

impl Language {
    /// Return the language with the language code name. (ex. "ar", "de")
    pub fn as_code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Farsi => "fa",
            Language::Arabic => "ar",
            Language::Chinese => "zh",
            Language::French => "fr",
            Language::German => "de",
            Language::Italian => "it",
            Language::Japanese => "ja",
            Language::Portuguese => "pt",
            Language::Russian => "ru",
            Language::Spanish => "es",
        }
    }

    /// Create a Language from &str like "en", "French" or "ژاپنی". Case Doesn't matter.
    pub fn from(s: &str) -> crate::Result<Self> {
        Self::from_str(s).map_err(|e| e.into())
    }
}

pub trait Translate: AsRef<str> {
    const BASE_URL: &'static str =
        "https://translate.google.com/translate_a/single?&client=gtx&sl=auto";
    const USER_AGENT_VALUE:&'static str = "Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36";

    fn translate(&self, target: Language) -> crate::Result<String> {
        let url = format!(
            "{url}&tl={target}&hl={target}&dt=t&text={text}",
            url = Self::BASE_URL,
            target = target.as_code(),
            text = &self.as_ref(),
        );
        reqwest::blocking::Client::new()
            .get(url)
            .header(USER_AGENT, Self::USER_AGENT_VALUE)
            .send()?
            .text()
            .map_err(|e| e.to_string())
            .and_then(|s| {
                match s
                    .find('"')
                    .map(|i| i + 1)
                    .and_then(|i| s[i..].find('"').map(|i2| (i, i2 + i)))
                {
                    Some((start, end)) if start != 0 && end != start => {
                        Ok(s[start..end].to_owned())
                    }
                    _ => Err("Does Not Exist".to_string()),
                }
            })
            .map_err(|e| e.into())
    }
}

impl_trait_for_string_types!(Translate);

#[cfg(test)]
mod translate_test {
    use super::{Language, Translate};

    #[test]
    fn translate() {
        assert_eq!(
            "سلام دنیا",
            r#"Hello, World"#.translate(Language::Farsi).unwrap()
        );
    }
}
