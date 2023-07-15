pub mod generated_manifests;
use std::{env, path::PathBuf};

use common::{
    tracing::info,
    traits::{DeserializationError, Tomlable},
};
use game::manifest::GameManifest;
use generated_manifests::load;
use peripheral::manifest::PeripheralManifest;

fn get_crate_path() -> PathBuf {
    env::var("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap()
}

pub struct Fixtures {}

impl Fixtures {
    pub fn peripheral(key: &str) -> Result<PeripheralManifest, DeserializationError> {
        // watertribe.card_reader
        // examples/watertribe.card_reader/watertribe.card_reader.lock.toml

        let file_name = format!("examples/peripherals/{key}/{key}.lock.toml");

        info!("{file_name}");

        let toml = load(&file_name).unwrap();

        PeripheralManifest::from_toml(toml)
    }

    pub fn game(key: &str) -> Result<GameManifest, DeserializationError> {
        let path = get_crate_path()
            .join("games")
            .join(key)
            .join("/game.lock.toml");
        GameManifest::from_toml_file(path)
    }
}
