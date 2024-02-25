use common::data::serialization::Jsonable;

use crate::examples::Example;
use crate::specs::Feature as SpecFeature;

pub struct Feature {
    json: String,
}

impl Example for Feature {
    type BuiltValue = SpecFeature;

    fn json(&self) -> &str {
        &self.json
    }
}

impl Feature {
    pub fn min() -> Self {
        Self {
            json: r#"
            {
                "key": "min"
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
}

#[cfg(test)]
mod test {
    use crate::examples;
    use crate::examples::features::Feature;

    #[test]
    fn it_serializes_min() {
        examples::run_example_round_trip_test(Feature::min);
    }

    #[test]
    fn it_serializes_led() {
        examples::run_example_round_trip_test(Feature::led);
    }
}
