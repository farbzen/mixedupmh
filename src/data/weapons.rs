use rand::random_range;
use serde::Deserialize;
use std::{env::current_dir, fs::read_to_string};

const PATH_WEAPON_FILE: &str = "weapons.json";

#[derive(Debug, Deserialize)]
pub struct Weapons {
    weapons: Vec<Weapon>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Weapon {
    pub name: String,
}

pub async fn load_weapons() -> Weapons {
    let path = current_dir()
        .unwrap_or_default()
        .join(super::PATH_JSON_FILES)
        .join(PATH_WEAPON_FILE);

    let file = read_to_string(path).expect("Unable to read weapon file!");

    serde_json::from_str(&file).unwrap_or(Weapons { weapons: vec![] })
}

pub async fn roll_weapon() -> Weapon {
    let weapons = load_weapons().await.weapons;
    let randint = random_range(0..weapons.len());

    weapons[randint].clone()
}
