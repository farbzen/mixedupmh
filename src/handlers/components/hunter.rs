use askama::Template;
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use tower_cookies::Cookies;

use crate::{
    cookies::{
        DEFAULT_HUNTERNAME, HUNTERNAME, WEAPONNAME, get_cookie_value,
        set_cookie,
    },
    data::{
        language::{LanguageDictionary, language_dictionary},
        weapons::{Weapon, roll_weapon},
    },
    handlers::errors::ERROR_RENDER,
};

#[derive(Template)]
#[template(path = "components/hunter.html")]
struct HunterComponent {
    lang: LanguageDictionary,
    hunter: String,
    number: String,
    weapon: Weapon,
}

pub async fn hunter(
    cookies: Cookies,
    Path(number): Path<String>,
) -> impl IntoResponse {
    let hunter = get_cookie_value(
        &cookies,
        &(HUNTERNAME.to_string() + &number),
        DEFAULT_HUNTERNAME,
    )
    .await;

    let weapon = roll_weapon().await;
    set_cookie(&cookies, &(WEAPONNAME.to_string() + &number), &weapon.name);

    let template = HunterComponent {
        lang: language_dictionary(&cookies).await,
        hunter,
        number,
        weapon,
    };
    let reply = template.render().unwrap_or(ERROR_RENDER.to_string());

    (StatusCode::OK, reply.into_response())
}
