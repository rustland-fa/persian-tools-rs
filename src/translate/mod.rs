use reqwest::header::USER_AGENT;
use strum::ToString;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PUNCH: Regex = Regex::new(r"[[:punc:]]").unwrap();
}

#[derive(Debug, ToString)]
pub enum Language {
    // TODO
    fa,
}

pub trait Translate: AsRef<str> {
    const BASE_URL: &'static str =
        "https://translate.google.com/translate_a/single?&client=gtx&sl=auto";
    const USER_AGENT_VALUE:&'static str = "Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36";

    fn translate(&self, target: Language) -> crate::Result<String> {
        let url = format!(
            "{url}&tl={target}&hl={target}&dt=t&text={text}",
            url = Self::BASE_URL,
            target = target.to_string(),
            text = &self.as_ref(),
        );
        reqwest::blocking::Client::new()
            .get(url)
            .header(USER_AGENT, Self::USER_AGENT_VALUE)
            .send()?
            .text()
            .map_err(|e| e.to_string())
            .and_then(|s| {
                s.replace("[", "")
                    .replace("]", "")
                    .split(",")
                    //TODO FIX BUG 
                    .next()
                    .map(|n| n.to_string())
                    .ok_or_else(|| "Does Not Exis".into())
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
        let salam = "Hello".translate(Language::fa).unwrap();
        println!("{}", salam);
        assert_eq!("سلام", salam);
    }
}
