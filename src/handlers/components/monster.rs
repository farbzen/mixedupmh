use askama::Template;
use axum::{http::StatusCode, response::IntoResponse};
use tower_cookies::Cookies;

use crate::{
    cookies::{CAPTURE, MONSTERNAME, TEMPER, set_cookie},
    data::{
        language::{LanguageDictionary, language_dictionary},
        monsters::{Monster, roll_capture, roll_monster, roll_temper},
    },
};

#[derive(Template)]
#[template(path = "components/monster.html")]
struct MonsterComponent {
    lang: LanguageDictionary,
    monster: Monster,
    capture: bool,
    temper: String,
}

pub async fn monster(cookies: Cookies) -> impl IntoResponse {
    let monster = roll_monster().await;
    set_cookie(&cookies, MONSTERNAME, &monster.name);

    let temper = roll_temper(&monster).await;
    set_cookie(&cookies, TEMPER, &temper);

    let capture = roll_capture(&monster).await;
    set_cookie(&cookies, CAPTURE, &capture.to_string());

    let template = MonsterComponent {
        lang: language_dictionary(&cookies).await,
        temper,
        monster,
        capture,
    };
    let reply = template.render().unwrap();

    (StatusCode::OK, reply.into_response())
}
