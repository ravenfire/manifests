use std::{env, path::PathBuf};

use common::traits::{DeserializationError, Tomlable};
use peripheral::manifest::PeripheralManifest;

fn get_crate_path() -> PathBuf {
    env::var("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap()
}

pub struct Fixtures {}

impl Fixtures {
    pub fn peripheral(key: &str) -> Result<PeripheralManifest, DeserializationError> {
        let path = get_crate_path().join("peripherals").join(key).join(".toml");
        PeripheralManifest::from_toml_file(path)
    }
}
