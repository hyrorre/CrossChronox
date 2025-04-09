use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Mode {
    keys: BTreeMap<String, Vec<i32>>,
    features: BTreeMap<String, Vec<String>>,
}

impl Mode {
    pub fn new() -> Self {
        Mode {
            keys: BTreeMap::new(),
            features: BTreeMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputManager {
    modes: BTreeMap<String, Mode>,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            modes: Self::load().unwrap_or_default(),
        }
    }

    fn load() -> Result<BTreeMap<String, Mode>, toml::de::Error> {
        toml::from_str(
            std::fs::read_to_string("CrossChronoxData/Config/KeyConfig.toml")
                .unwrap_or("".to_string())
                .as_str(),
        )
    }

    pub fn update(&mut self) {
        // Update input state
    }
}
