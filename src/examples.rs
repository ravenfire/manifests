use common::data::serialization::Jsonable;

use crate::game::GameManifest;
use crate::peripheral::PeripheralManifest;

pub struct Peripheral {
    json: String,
}

impl Peripheral {
    pub fn rf_dice_pad() -> Peripheral {
        Self {
            json: include_str!("../examples/peripherals/rf.dice_pad/rf.dice_pad.json").to_string(),
        }
    }

    pub fn rf_screen() -> Peripheral {
        Self {
            json: include_str!("../examples/peripherals/rf.screen/rf.screen.json").to_string(),
        }
    }

    pub fn watertribe_card_reader() -> Peripheral {
        Self {
            json: include_str!(
                "../examples/peripherals/watertribe.card_reader/watertribe.card_reader.json"
            )
            .to_string(),
        }
    }

    pub fn json(&self) -> &str {
        &self.json
    }

    pub fn build(&self) -> PeripheralManifest {
        PeripheralManifest::from_json(self.json()).expect("Failed to build PeripheralManifest")
    }
}

pub struct Game {
    json: String,
}

impl Game {
    // pub fn spellbinder() -> crate::examples::Game {
    //     Self {
    //         json: include_str!("../examples/games/spellbinder/game.lock.json").to_string(),
    //     }
    // }
    //
    // pub fn tictactoe() -> crate::examples::Game {
    //     Self {
    //         json: include_str!("../examples/games/tictactoe/game.lock.json").to_string(),
    //     }
    // }

    pub fn simple_battle() -> crate::examples::Game {
        Self {
            json: include_str!("../examples/games/simple_battle/simple_battle.json").to_string(),
        }
    }

    pub fn json(&self) -> &str {
        &self.json
    }

    pub fn build(&self) -> GameManifest {
        GameManifest::from_json(self.json()).expect("Failed to build GameManifest")
    }
}
