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
    attachments: Vec<String>,
    accuracy: i8,
    recoil: i8,
    weight: f32,
    size: i8,
    other: Vec<String>,
}

pub static GUNS: LazyLock<Vec<Gun>> = LazyLock::new(|| self::fill_guns());

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

pub fn get_gun(name: String) -> Gun {
    let mut gun: Gun = Gun {
        name: "ADAR 2-15".to_string(),
        cartridge: "5.56x45mm".to_string(),
        range: 500,
        semi: true,
        full: 0,
        burst: [0, 0].to_vec(),
        ammo: "FMJ".to_string(),
        attachments: ["Sight".to_string(), "Compensator".to_string()].to_vec(),
        accuracy: 0,
        recoil: 0,
        weight: 3.0,
        size: 10,
        other: [].to_vec(),
    };

    for value in &*GUNS {
        if value.name == name {
            gun = value.clone();
        }
    }

    return gun;
}
