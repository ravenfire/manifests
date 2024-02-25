use crate::examples::Example;

pub struct Spec {
    json: String,
}

impl Example for Spec {
    type BuiltValue = crate::specs::Spec;

    fn json(&self) -> &str {
        &self.json
    }
}

impl Spec {
    pub fn min() -> Self {
        Self {
            json: include_str!("../../examples/specs/min.json").to_string(),
        }
    }

    pub fn card() -> Self {
        Self {
            json: include_str!("../../examples/specs/card/card.json").to_string(),
        }
    }

    pub fn dice() -> Self {
        Self {
            json: include_str!("../../examples/specs/dice/dice.json").to_string(),
        }
    }

    pub fn screen() -> Self {
        Self {
            json: include_str!("../../examples/specs/screen/screen.json").to_string(),
        }
    }

    pub fn json(&self) -> &str {
        &self.json
    }
}

#[cfg(test)]
mod test {
    use crate::examples;
    use crate::examples::specs::Spec;

    #[test]
    fn it_serializes_min() {
        examples::run_example_round_trip_test(Spec::min);
    }

    #[test]
    fn it_serializes_card() {
        examples::run_example_round_trip_test(Spec::card);
    }

    #[test]
    fn it_serializes_dice() {
        examples::run_example_round_trip_test(Spec::dice);
    }

    #[test]
    fn it_serializes_screen() {
        examples::run_example_round_trip_test(Spec::screen);
    }
}
