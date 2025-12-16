use crate::{data::language::{LanguageDictionary, language_dictionary}, handlers::errors::ERROR_RENDER};
use askama::Template;
use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};
use tower_cookies::Cookies;

pub fn route_modals() -> Router {
    Router::new()
        .route("/empty", get(empty))
        .route("/legal", get(legal))
}

#[derive(Template)]
#[template(path = "components/modals/legal.html")]
pub struct LegalModal {
    lang: LanguageDictionary,
}

#[derive(Template)]
#[template(path = "components/modals/empty.html")]
pub struct EmptyModal {}

pub async fn legal(cookies: Cookies) -> impl IntoResponse {
    let template = LegalModal {
        lang: language_dictionary(&cookies).await,
    };
    let reply = template.render().unwrap_or(ERROR_RENDER.to_string());

    (StatusCode::OK, reply.into_response())
}

pub async fn empty() -> impl IntoResponse {
    let template = EmptyModal {};
        let reply = template.render().unwrap_or(ERROR_RENDER.to_string());

    (StatusCode::OK, reply.into_response())
}
