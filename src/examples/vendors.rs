use crate::examples::Example;

pub struct Vendor {
    json: String,
}

impl Example for Vendor {
    type BuiltValue = crate::vendor::Vendor;

    fn json(&self) -> &str {
        &self.json
    }
}

impl Vendor {
    pub fn min() -> Self {
        Self {
            json: include_str!("../../examples/vendor/min.json").to_string(),
        }
    }

    pub fn ravenfire() -> Self {
        Self {
            json: include_str!("../../examples/vendor/ravenfire.json").to_string(),
        }
    }

    pub fn json(&self) -> &str {
        &self.json
    }
}

#[cfg(test)]
mod test {
    use crate::examples;
    use crate::examples::vendors::Vendor;

    #[test]
    fn it_serializes_min() {
        examples::run_example_round_trip_test(Vendor::min);
    }

    #[test]
    fn it_serializes_ravenfire() {
        examples::run_example_round_trip_test(Vendor::ravenfire);
    }
}
