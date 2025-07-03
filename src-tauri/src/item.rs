use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, Value};

use std::env;
use std::fs;
use std::sync::LazyLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gun {
    name: String,
    cartridge: String,
    range: i16,
    semi: bool,
    full: i8,
    burst: Vec<i8>,
    ammo: String,
    max: i8,
    attachments: Vec<String>,
    accuracy: i8,
    recoil: i8,
    weight: f32,
    size: i8,
    other: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round {
    name: String,
    damage: String,
    penetration: i8,
    recoil: i8,
    accuracy: i8,
    other: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ammo {
    class: String,
    rounds: Vec<Round>,
}

pub static GUNS: LazyLock<Vec<Gun>> = LazyLock::new(|| self::fill_guns());
pub static AMMO: LazyLock<Vec<Ammo>> = LazyLock::new(|| self::fill_ammo());

fn fill_guns() -> Vec<Gun> {
    let mut path = env::current_dir().expect("Failed to retrieve path");
    path.push("data/guns.json");
    let data: String = fs::read_to_string(&path).expect("Failed to read from file");
    let stream = Deserializer::from_str(&data).into_iter::<Value>();

    let mut guns: Vec<Gun> = Vec::new();
    for value in stream {
        guns.push(Gun::deserialize(value.unwrap()).expect("Failed to convert"));
    }

    return guns;
}

fn fill_ammo() -> Vec<Ammo> {
    let mut path = env::current_dir().expect("Failed to retrieve path");
    path.push("data/ammo.json");
    let data: String = fs::read_to_string(&path).expect("Failed to read from file");
    let stream = Deserializer::from_str(&data).into_iter::<Value>();

    let mut ammo: Vec<Ammo> = Vec::new();
    for value in stream {
        ammo.push(Ammo::deserialize(value.unwrap()).expect("Failed to convert"));
    }

    return ammo;
}

pub fn get_gun_names() -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for value in &*GUNS {
        names.push(value.name.clone());
    }

    return names;
}

pub fn get_gun(name: String) -> Gun {
    let mut gun: Gun = Gun {
        name: "ADAR 2-15".to_string(),
        cartridge: "5.56x45mm".to_string(),
        range: 500,
        semi: true,
        full: 0,
        burst: [0, 0].to_vec(),
        ammo: "FMJ".to_string(),
        max: 30,
        attachments: ["Sight".to_string(), "Compensator".to_string()].to_vec(),
        accuracy: 0,
        recoil: 0,
        weight: 3.0,
        size: 10,
        other: vec![],
    };

    for value in &*GUNS {
        if value.name == name {
            gun = value.clone();
        }
    }

    return gun;
}

pub fn get_ammo(ammo_name: String, round_name: String) -> Round {
    let mut result: Round = Round {
        name: "Warmageddon".to_string(),
        damage: "11d8".to_string(),
        penetration: 0,
        recoil: -5,
        accuracy: 5,
        other: vec![],
    };

    for ammo in &*AMMO {
        if ammo.class == ammo_name {
            for round in &ammo.rounds {
                if round.name == round_name {
                    result = round.clone();
                }
            }
        }
    }

    return result;
}
