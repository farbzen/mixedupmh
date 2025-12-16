use rand::{random_bool, random_range};
use serde::Deserialize;
use std::{env::current_dir, fs::read_to_string};

const PATH_MONSTER_FILE: &str = "monsters.json";

#[derive(Debug, Deserialize)]
pub struct Monsters {
    monsters: Vec<Monster>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Monster {
    pub name: String,
    pub capturable: bool,
    pub tempered: bool,
    pub arch_tempered: bool,
}

pub async fn load_monsters() -> Monsters {
    let path = current_dir()
        .unwrap_or_default()
        .join(super::PATH_JSON_FILES)
        .join(PATH_MONSTER_FILE);

    let file = read_to_string(path).expect("Unable to read monster file!");

    serde_json::from_str(&file).unwrap_or(Monsters { monsters: vec![] })
}

pub async fn roll_monster() -> Monster {
    let monsters = load_monsters().await.monsters;
    let randint = random_range(0..monsters.len());

    monsters[randint].clone()
}

pub async fn roll_capture(monster: &Monster) -> bool {
    let odds: f64 = 0.5;

    let mut capture = false;

    if monster.capturable {
        capture = random_bool(odds)
    };

    capture
}

pub async fn roll_temper(monster: &Monster) -> String {
    let mut temper = 0;

    if monster.arch_tempered {
        temper = random_range(0..3);
    } else if monster.tempered {
        temper = random_range(0..2);
    }

    temper.to_string()
}
