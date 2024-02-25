use common::data::schema::Property as DataProperty;
use common::data::serialization::Jsonable;

use crate::examples::Example;

pub struct Property {
    json: String,
}

impl Example for Property {
    type BuiltValue = DataProperty;

    fn json(&self) -> &str {
        &self.json
    }
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
    pub fn min() -> Self {
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
}

#[cfg(test)]
mod test {
    use crate::examples;
    use crate::examples::properties::Property;

    #[test]
    fn it_serializes_min() {
        examples::run_example_round_trip_test(Property::min);
    }

    #[test]
    fn it_serializes_facing() {
        examples::run_example_round_trip_test(Property::facing);
    }

    #[test]
    fn it_serializes_weapon() {
        examples::run_example_round_trip_test(Property::weapon);
    }
}
