use lazy_static::lazy_static;
use regex::Regex;
use reqwest::header::USER_AGENT;
use std::str::FromStr;
use strum::ToString;

lazy_static! {
    static ref DOUBLE_QUOTES: Regex = Regex::new(r#""(.*?)""#).unwrap();
}

/// Languages that can used for input and output of the [`translate`] function.
#[derive(Debug, Clone, PartialEq, Copy, Hash, ToString)]
pub enum Language {
    English,
    Farsi,
    Arabic,
    Chinese,
    French,
    German,
    Italian,
    Japanese,
    Portuguese,
    Russian,
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

    /// Create a Language from &str like "en" or "French". Case Doesn't matter.
    pub fn from(s: &str) -> crate::Result<Self> {
        Self::from_str(s).map_err(|e| e.into())
    }
}

impl FromStr for Language {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_string().to_lowercase().as_str() {
            "en" => Ok(Language::English),
            "fa" => Ok(Language::Farsi),
            "ar" => Ok(Language::Arabic),
            "zh" => Ok(Language::Chinese),
            "fr" => Ok(Language::French),
            "de" => Ok(Language::German),
            "it" => Ok(Language::Italian),
            "pt" => Ok(Language::Portuguese),
            "ru" => Ok(Language::Russian),
            "es" => Ok(Language::Spanish),
            "ja" => Ok(Language::Japanese),
            "english" => Ok(Language::English),
            "farsi" => Ok(Language::Farsi),
            "arabic" => Ok(Language::Arabic),
            "chinese" => Ok(Language::Chinese),
            "french" => Ok(Language::French),
            "german" => Ok(Language::German),
            "italian" => Ok(Language::Italian),
            "portuguese" => Ok(Language::Portuguese),
            "russian" => Ok(Language::Russian),
            "spanish" => Ok(Language::Spanish),
            "japanese" => Ok(Language::Japanese),
            &_ => Err("input invalid!"),
        }
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
                if let Some(c) = DOUBLE_QUOTES.captures_iter(&s).next() {
                    let content = c[0].trim();
                    let len = content.len();
                    Ok(content[1..len - 1].to_string())
                } else {
                    Err("Does Not Exist".to_string())
                }
            })
            .map_err(|e| e.into())
    }
}

impl Translate for String {}

impl Translate for str {}

#[cfg(test)]
mod translate {
    use super::{Language, Translate};

    #[test]
    fn translate() {
        assert_eq!(
            "سلام دنیا",
            r#"Hello, World"#.translate(Language::Farsi).unwrap()
        );
    }
}
