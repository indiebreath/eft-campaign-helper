use serde::{Deserialize, Serialize};

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

pub fn assault_to_json() -> Gun {
    let data = r#"{
        "name": "ADAR 2-15",
        "cartridge": "5.56x45mm",
        "range": 500,
        "semi": true,
        "full": 0,
        "burst": [0, 0],
        "ammo": "FMJ",
        "attachments": ["Sight", "Compensator"],
        "accuracy": 0,
        "recoil": 0,
        "weight": 3.0,
        "size": 10,
        "other": []
    }"#;

    let assault: Gun = serde_json::from_str(data).expect("Failed");
    return assault;
}
