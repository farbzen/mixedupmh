use serde_json::{Value, from_str};
use std::{env::current_dir, fs::read_to_string};
use tower_cookies::Cookies;

use crate::cookies::{LANGUAGE, get_cookie_value, set_cookie};

const PATH_LANGUAGE_FILES: &str = "language";

const LANGUAGE_EN: &str = "en";
const LANGUAGE_DE: &str = "de";
const LANGUAGES_SUPPORTED: [&str; 2] = [LANGUAGE_EN, LANGUAGE_DE];
const DEFAULT_LANGUAGE: &str = LANGUAGE_EN;

pub struct LanguageDictionary {
    pub lang: String,
    content: Value,
}

impl<'a> LanguageDictionary {
    pub fn get(&'a self, key: &'a str) -> &'a str {
        self.content[key].as_str().unwrap_or(key)
    }
}

pub async fn language_dictionary(cookies: &Cookies) -> LanguageDictionary {
    let language = get_cookie_value(cookies, LANGUAGE, DEFAULT_LANGUAGE).await;

    let path = current_dir()
        .unwrap_or_default()
        .join(super::PATH_JSON_FILES)
        .join(PATH_LANGUAGE_FILES)
        .join(language.to_string() + ".json");

    let file = read_to_string(path).expect("Unable to read language file!");

    LanguageDictionary {
        lang: language,
        content: from_str(&file).unwrap_or(Value::Null),
    }
}

pub async fn set_language(cookies: &Cookies, language: &str) {
    for lang in LANGUAGES_SUPPORTED {
        if language == lang {
            set_cookie(cookies, LANGUAGE, language);
            break;
        }
    }
}
