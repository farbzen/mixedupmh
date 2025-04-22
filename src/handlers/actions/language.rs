use crate::data::language::set_language;
use axum::{extract::Path, response::Redirect};
use tower_cookies::Cookies;

pub async fn switch_language(
    cookies: Cookies,
    Path(language): Path<String>,
) -> Redirect {
    set_language(&cookies, &language).await;

    Redirect::to("/")
}
