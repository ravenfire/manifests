//! This module provides a set of example data for testing and documentation purposes.
//! Examples panic on purpose instead of using Result<>
use common::data::schema::Property as DataProperty;
use common::data::serialization::Jsonable;

use crate::game::GameManifest;
use crate::peripheral::PeripheralManifest;
use crate::specs::Feature as SpecFeature;

pub struct Peripheral {
    json: String,
}

impl Peripheral {
    pub fn min() -> Self {
        Self {
            json: include_str!("../examples/peripherals/min.json").to_string(),
        }
    }

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
    pub fn min() -> Self {
        Self {
            json: include_str!("../examples/min.json").to_string(),
        }
    }

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
    pub fn min() -> Self {
        Self {
            json: include_str!("../examples/specs/min.json").to_string(),
        }
    }

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
    pub fn min() -> Self {
        Self {
            json: include_str!("../examples/vendor/min.json").to_string(),
        }
    }

    pub fn ravenfire() -> Self {
        Self {
            json: include_str!("../examples/vendor/ravenfire.json").to_string(),
        }
    }

    pub fn json(&self) -> &str {
        &self.json
    }

    pub fn build(&self) -> crate::vendor::Vendor {
        crate::vendor::Vendor::from_json(self.json()).expect("Failed to build Vendor")
    }
}

// These are component parts and don't deserve their own json files in the examples directory

pub struct Feature {
    json: String,
}

impl Feature {
    pub fn rfid() -> Self {
        Self {
            json: r#"
            {
                "key": "rfid"
            }
            "#
            .to_string(),
        }
    }
    pub fn led() -> Self {
        Self {
            json: r#"
            {
                "key": "led",
                "titles": {
                    "en": "LED"
                },
                "descriptions": {
                    "en": "Is an LED Light On?"
                },
                "properties": [
                    {
                        "key": "color",
                        "data_type": "String",
                        "titles": {
                            "en": "Color"
                        },
                        "descriptions": {
                            "en": "What color is the LED?"
                        },
                        "enumerations": [
                            {"String": "red"},
                            {"String": "green"},
                            {"String": "blue"}
                        ]
                    }
                ]
            }
            "#
            .to_string(),
        }
    }

    pub fn json(&self) -> &str {
        &self.json
    }

    pub fn build(&self) -> SpecFeature {
        SpecFeature::from_json(self.json()).expect("Failed to build Feature")
    }
}

pub struct Property {
    json: String,
}

impl Property {
    pub fn facing() -> Self {
        Self {
            json: r#"
            {
                "key": "facing",
                "data_type": "String",
                "enumerations": [
                    {"String": "up"},
                    {"String": "down"}
                ]
            }
            "#
            .to_string(),
        }
    }
    pub fn strength() -> Self {
        Self {
            json: r#"
            {
                "key": "strength",
                "data_type": "Float"
            }
            "#
            .to_string(),
        }
    }

    pub fn weapon() -> Self {
        Self {
            json: r#"
            {
                "key": "weapon",
                "data_type": "Playable",
                "optional": true,
                "titles": {
                    "en": "Playable"
                },
                "descriptions": {
                    "en": "Which weapon do you have?"
                }
            }
            "#
            .to_string(),
        }
    }

    pub fn json(&self) -> &str {
        &self.json
    }

    pub fn build(&self) -> DataProperty {
        DataProperty::from_json(self.json()).expect("Failed to build Property")
    }
}
