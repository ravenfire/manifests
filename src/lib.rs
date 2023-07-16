pub mod generated_manifests;
use std::{env, path::PathBuf};

use common::traits::{DeserializationError, Tomlable};
use game::manifest::GameManifest;
use generated_manifests::load;
use peripheral::manifest::PeripheralManifest;

fn get_crate_path() -> PathBuf {
    env::var("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap()
}

pub struct Fixtures {}

// @todo: This needs to be able to return both the built peripheral and the raw toml
impl Fixtures {
    pub fn peripheral(key: &str) -> Result<PeripheralManifest, DeserializationError> {
        let file_name = format!("examples/peripherals/{key}/{key}.lock.toml");
        let toml = load(&file_name).expect(&format!("Could not find file: {}", file_name));

        PeripheralManifest::from_toml(toml)
    }

    pub fn game(key: &str) -> Result<GameManifest, DeserializationError> {
        let file_name = format!("examples/games/{key}/game.lock.toml");
        let toml = load(&file_name).unwrap();

        GameManifest::from_toml(toml)
    }
}
