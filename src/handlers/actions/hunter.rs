use crate::cookies::{HUNTERCOUNT, HUNTERNAME, set_cookie};
use axum::extract::Query;
use serde::Deserialize;
use tower_cookies::Cookies;

#[derive(Deserialize)]
pub struct HunterRename {
    name: String,
    number: String,
}

#[derive(Deserialize)]
pub struct HunterCount {
    count: String,
}

pub async fn rename_hunter(
    cookies: Cookies,
    Query(hunter_rename): Query<HunterRename>,
) {
    set_cookie(
        &cookies,
        &(HUNTERNAME.to_owned() + &hunter_rename.number),
        &hunter_rename.name,
    );
}

pub async fn hunter_count(cookies: Cookies, Query(count): Query<HunterCount>) {
    set_cookie(&cookies, HUNTERCOUNT, &count.count);
}
