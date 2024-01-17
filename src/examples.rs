use common::data::serialization::Tomlable;

use crate::game::GameManifest;
use crate::peripheral::PeripheralManifest;

pub struct Peripheral {
    toml: String,
}

impl Peripheral {
    pub fn rf_dice_pad() -> Peripheral {
        Self {
            toml: include_str!("../examples/peripherals/rf.dice_pad/rf.dice_pad.lock.toml")
                .to_string(),
        }
    }

    pub fn rf_screen() -> Peripheral {
        Self {
            toml: include_str!("../examples/peripherals/rf.screen/rf.screen.lock.toml").to_string(),
        }
    }

    pub fn spellbinder_world_builder() -> Peripheral {
        Self {
            toml: include_str!(
                "../examples/peripherals/spellbinder.world_builder/spellbinder.world_builder.lock.toml"
            )
                .to_string(),
        }
    }

    pub fn watertribe_card_reader() -> Peripheral {
        Self {
            toml: include_str!(
                "../examples/peripherals/watertribe.card_reader/watertribe.card_reader.lock.toml"
            )
            .to_string(),
        }
    }

    pub fn toml(&self) -> &str {
        &self.toml
    }

    pub fn build(&self) -> PeripheralManifest {
        PeripheralManifest::from_toml(self.toml()).expect("Failed to build PeripheralManifest")
    }
}

pub struct Game {
    toml: String,
}

impl crate::examples::Game {
    pub fn spellbinder() -> crate::examples::Game {
        Self {
            toml: include_str!("../examples/games/spellbinder/game.lock.toml").to_string(),
        }
    }

    pub fn tictactoe() -> crate::examples::Game {
        Self {
            toml: include_str!("../examples/games/tictactoe/game.lock.toml").to_string(),
        }
    }

    pub fn toml(&self) -> &str {
        &self.toml
    }

    pub fn build(&self) -> GameManifest {
        GameManifest::from_toml(self.toml()).expect("Failed to build GameManifest")
    }
}
