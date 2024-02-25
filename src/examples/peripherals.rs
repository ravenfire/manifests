use common::data::serialization::Jsonable;

use crate::examples::Example;
use crate::peripheral::PeripheralManifest;

pub struct Peripheral {
    json: String,
}

impl Example for Peripheral {
    type BuiltValue = PeripheralManifest;

    fn json(&self) -> &str {
        &self.json
    }
}

impl Peripheral {
    pub fn min() -> Self {
        Self {
            json: include_str!("../../examples/peripherals/min.json").to_string(),
        }
    }

    pub fn rf_card_reader() -> Self {
        Self {
            json: include_str!("../../examples/peripherals/rf.card_reader/rf.card_reader.json")
                .to_string(),
        }
    }

    pub fn rf_dice_pad() -> Self {
        Self {
            json: include_str!("../../examples/peripherals/rf.dice_pad/rf.dice_pad.json")
                .to_string(),
        }
    }

    pub fn rf_screen() -> Self {
        Self {
            json: include_str!("../../examples/peripherals/rf.screen/rf.screen.json").to_string(),
        }
    }

    pub fn watertribe_card_reader() -> Self {
        Self {
            json: include_str!(
                "../../examples/peripherals/watertribe.card_reader/watertribe.card_reader.json"
            )
            .to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::examples::run_example_round_trip_test;

    #[test]
    fn it_serializes_min() {
        run_example_round_trip_test(crate::examples::peripherals::Peripheral::min);
    }

    #[test]
    fn it_serializes_rf_card_reader() {
        run_example_round_trip_test(crate::examples::peripherals::Peripheral::rf_card_reader);
    }

    #[test]
    fn it_serializes_rf_dice_pad() {
        run_example_round_trip_test(crate::examples::peripherals::Peripheral::rf_dice_pad);
    }

    #[test]
    fn it_serializes_rf_screen() {
        run_example_round_trip_test(crate::examples::peripherals::Peripheral::rf_screen);
    }

    #[test]
    fn it_serializes_watertribe_card_reader() {
        run_example_round_trip_test(
            crate::examples::peripherals::Peripheral::watertribe_card_reader,
        );
    }
}
