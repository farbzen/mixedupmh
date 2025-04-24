use crate::{
    cookies::{
        CAPTURE, DEFAULT_CAPTURE, DEFAULT_HUNTERNAME, DEFAULT_MONSTERNAME,
        DEFAULT_TEMPER, DEFAULT_WEAPONNAME, HUNTERCOUNT, HUNTERNAME,
        MONSTERNAME, TEMPER, WEAPONNAME, get_cookie_value,
    },
    data::language::{LanguageDictionary, language_dictionary, set_language},
};
use askama::Template;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use chrono::Datelike;
use serde::Deserialize;
use tower_cookies::Cookies;

#[derive(Deserialize)]
pub struct LanguageSetter {
    language: Option<String>,
}

#[derive(Template)]
#[template(path = "pages/index.html")]
struct IndexPage {
    lang: LanguageDictionary,
    hunter_count: u8,
    hunter_names: Vec<String>,
    weapons: Vec<String>,
    monster: String,
    capture: bool,
    temper: String,
    year: String,
}

pub async fn index(
    cookies: Cookies,
    Query(language): Query<LanguageSetter>,
) -> impl IntoResponse {
    if let Some(lang) = language.language {
        set_language(&cookies, &lang).await;
    }

    let hunter_count = get_cookie_value(&cookies, HUNTERCOUNT, "4")
        .await
        .parse::<u8>()
        .unwrap_or(4);

    let mut hunter_names: Vec<String> = Vec::new();
    let mut weapons: Vec<String> = Vec::new();

    for hunter in 1..5 {
        hunter_names.push(
            get_cookie_value(
                &cookies,
                &(HUNTERNAME.to_string() + &hunter.to_string()),
                DEFAULT_HUNTERNAME,
            )
            .await,
        );

        weapons.push(
            get_cookie_value(
                &cookies,
                &(WEAPONNAME.to_string() + &hunter.to_string()),
                DEFAULT_WEAPONNAME,
            )
            .await,
        );
    }

    let monster =
        get_cookie_value(&cookies, MONSTERNAME, DEFAULT_MONSTERNAME).await;

    let year = chrono::Utc::now().year().to_string();

    let template = IndexPage {
        lang: language_dictionary(&cookies).await,
        hunter_count,
        hunter_names,
        weapons,
        monster,
        capture: get_cookie_value(&cookies, CAPTURE, DEFAULT_CAPTURE)
            .await
            .parse::<bool>()
            .unwrap_or(false),
        temper: get_cookie_value(&cookies, TEMPER, DEFAULT_TEMPER).await,
        year,
    };
    let html = template.render().unwrap();

    (StatusCode::OK, Html(html).into_response())
}
