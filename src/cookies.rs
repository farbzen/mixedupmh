use tower_cookies::{Cookie, Cookies};

pub const LANGUAGE: &str = "language";
pub const HUNTERNAME: &str = "hunter_name";
pub const HUNTERCOUNT: &str = "hunter_count";
pub const WEAPONNAME: &str = "weapon_name";
pub const MONSTERNAME: &str = "monster_name";
pub const CAPTURE: &str = "capture";
pub const TEMPER: &str = "temper";

pub const DEFAULT_HUNTERNAME: &str = "New Hunter";
pub const DEFAULT_WEAPONNAME: &str = "sword_and_shield";
pub const DEFAULT_MONSTERNAME: &str = "chatacabra";
pub const DEFAULT_CAPTURE: &str = "false";
pub const DEFAULT_TEMPER: &str = "0";

const COOKIE_SAMESITE: &str = "SameSite=Lax";
const COOKIE_PATH: &str = "Path=/";
const COOKIE_MAX_AGE: &str = "Max-Age=10000000";
const COOKIE_SECURE: &str = "Secure";

pub async fn get_cookie_value(
    cookies: &Cookies,
    name: &str,
    default: &str,
) -> String {
    match cookies.get(name) {
        Some(c) => c.value().to_string(),
        None => {
            set_cookie(cookies, name, default);
            default.to_string()
        }
    }
}

pub fn set_cookie(cookies: &Cookies, name: &str, value: &str) {
    cookies.add(
        Cookie::parse(format!(
            "{} = {}; {}; {}; {}; {}",
            name,
            value,
            COOKIE_SAMESITE,
            COOKIE_PATH,
            COOKIE_MAX_AGE,
            COOKIE_SECURE
        ))
        .unwrap_or_else(|_| {
            println!("Error while parsing the new cookie!");
            Cookie::new("", "")
        }),
    );
}
