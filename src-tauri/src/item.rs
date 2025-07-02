use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, Value};

use std::env;
use std::fs;
use std::sync::LazyLock;

#[derive(Debug, Serialize, Deserialize)]
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
