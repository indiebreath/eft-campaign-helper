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
    class: Vec<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Magazine {
    name: String,
    cartridge: Vec<String>,
    fits: Vec<String>,
    capacity: i8,
    size: i8,
    weight: f32,
}

pub static GUNS: LazyLock<Vec<Gun>> = LazyLock::new(|| self::fill_guns());
pub static AMMO: LazyLock<Vec<Ammo>> = LazyLock::new(|| self::fill_ammo());
pub static MAGAZINES: LazyLock<Vec<Magazine>> = LazyLock::new(|| self::fill_magazines());

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

pub fn fill_magazines() -> Vec<Magazine> {
    let mut path = env::current_dir().expect("Failed to retrieve path");
    path.push("data/magazines.json");
    let data: String = fs::read_to_string(&path).expect("Failed to read from file");
    let stream = Deserializer::from_str(&data).into_iter::<Value>();

    let mut magazines: Vec<Magazine> = Vec::new();
    for value in stream {
        magazines.push(Magazine::deserialize(value.unwrap()).expect("Failed to convert"));
    }

    return magazines;
}

pub fn get_gun_names() -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for value in &*GUNS {
        names.push(value.name.clone());
    }
    return names;
}

pub fn get_gun(name: String) -> Gun {
    let pos = &GUNS
        .clone()
        .into_iter()
        .position(|n| n.name == name)
        .unwrap();
    return GUNS[*pos].clone();
}

pub fn get_ammo_names(ammo_name: String) -> Vec<String> {
    let pos = &AMMO
        .clone()
        .into_iter()
        .position(|c| c.class == ammo_name)
        .unwrap();

    let mut names: Vec<String> = Vec::new();
    for value in &*AMMO[*pos].rounds {
        names.push(value.name.clone());
    }

    return names;
}

pub fn get_ammo(ammo_name: String, round_name: String) -> Round {
    let class_pos = &AMMO
        .clone()
        .into_iter()
        .position(|c| c.class == ammo_name)
        .unwrap();
    let round_pos = &AMMO[*class_pos]
        .rounds
        .clone()
        .into_iter()
        .position(|n| n.name == round_name)
        .unwrap();

    return AMMO[*class_pos].rounds[*round_pos].clone();
}

pub fn get_magazine_names(gun_name: String) -> Vec<String> {
    let magazines = vec![MAGAZINES
        .clone()
        .into_iter()
        .filter(|f| f.fits.contains(&gun_name))
        .collect::<Vec<Magazine>>()];

    let mut names: Vec<String> = Vec::new();
    for value in &magazines[0] {
        names.push(value.name.clone());
    }

    return names;
}

pub fn get_magazine(magazine_name: String) -> Magazine {
    let pos = &MAGAZINES
        .clone()
        .into_iter()
        .position(|n| n.name == magazine_name)
        .unwrap();
    return MAGAZINES[*pos].clone();
}
