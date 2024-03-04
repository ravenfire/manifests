//! This module provides a set of example data for testing and documentation purposes.
//! Examples panic on purpose instead of using Result<>
use std::fmt::Debug;

use common::data::serialization::{Jsonable, Streamable, Tomlable};

pub mod features;
pub mod games;
pub mod peripherals;
pub mod properties;
pub mod specs;
pub mod vendors;

pub trait Example {
    type BuiltValue: Jsonable<Entity = Self::BuiltValue>
        + Tomlable<Entity = Self::BuiltValue>
        + Streamable<Entity = Self::BuiltValue>
        + PartialEq
        + Debug;

    fn json(&self) -> &str;
    fn build(&self) -> Self::BuiltValue {
        Self::BuiltValue::from_json(self.json()).expect("Failed to build Feature")
    }
}

#[cfg(test)]
fn run_example_round_trip_test<T, F>(setup: F)
where
    T: Example,
    F: Fn() -> T,
{
    // We start with an instance of T
    let example = setup();

    // Build the 'BuiltValue' instance
    let built = example.build();

    // Convert to JSON
    let serialized = built.to_json().unwrap();

    // Convert back from JSON to 'BuiltValue'
    let deserialized: T::BuiltValue = T::BuiltValue::from_json(&serialized).unwrap();

    assert_eq!(deserialized, built);
}
