//! This module provides a set of example data for testing and documentation purposes.
//! Examples panic on purpose instead of using Result<>
use common::data::serialization::Jsonable;

use crate::game::GameManifest;
use crate::peripheral::PeripheralManifest;

pub struct Peripheral {
    json: String,
}

impl Peripheral {
    pub fn rf_card_reader() -> Self {
        Self {
            json: include_str!("../examples/peripherals/rf.card_reader/rf.card_reader.json")
                .to_string(),
        }
    }

    pub fn rf_dice_pad() -> Self {
        Self {
            json: include_str!("../examples/peripherals/rf.dice_pad/rf.dice_pad.json").to_string(),
        }
    }

    pub fn rf_screen() -> Self {
        Self {
            json: include_str!("../examples/peripherals/rf.screen/rf.screen.json").to_string(),
        }
    }

    pub fn watertribe_card_reader() -> Self {
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
    pub fn simple_battle() -> Self {
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

pub struct Spec {
    json: String,
}

impl Spec {
    pub fn card() -> Self {
        Self {
            json: include_str!("../examples/specs/card/card.json").to_string(),
        }
    }

    pub fn dice() -> Self {
        Self {
            json: include_str!("../examples/specs/dice/dice.json").to_string(),
        }
    }

    pub fn screen() -> Self {
        Self {
            json: include_str!("../examples/specs/screen/screen.json").to_string(),
        }
    }

    pub fn json(&self) -> &str {
        &self.json
    }

    pub fn build(&self) -> crate::specs::Spec {
        crate::specs::Spec::from_json(self.json()).expect("Failed to build Spec")
    }
}

pub struct Vendor {
    json: String,
}

impl Vendor {
    pub fn ravenfire() -> Self {
        Self {
            json: include_str!("../examples/vendor/ravenfire.json").to_string(),
        }
    }

    pub fn ravenfire_min() -> Self {
        Self {
            json: include_str!("../examples/vendor/ravenfire-min.json").to_string(),
        }
    }

    pub fn json(&self) -> &str {
        &self.json
    }

    pub fn build(&self) -> crate::vendor::Vendor {
        crate::vendor::Vendor::from_json(self.json()).expect("Failed to build Vendor")
    }
}
