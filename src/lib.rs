
//! Traits and common data types used for manifests.
//!
//! Manifests live in their various crates (peripherals, games, etc).
pub mod game;
pub mod peripheral;
pub mod examples;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

use getset::{Getters, Setters};
use isolang::Language;
use serde::{de, Deserialize, Deserializer, Serialize};

use common::str;

use crate::data::serialization::{Streamable as StreamableTrait, Tomlable as TomlableTrait};
use crate::macros::{Jsonable, Streamable, Tomlable};
use crate::url::Url;

/// Represents a key/value pair of a language code and a string.
/// Used for storing localized strings. For things like names, descriptions, etc.
pub type LanguageMap = HashMap<Language, String>;

/// Trait for representing a manifest with serialization and streaming capabilities.
///
/// This trait provides functionalities for validating, locking, and saving manifest data.
/// It integrates `TomlableTrait` and `StreamableTrait` to support serialization to TOML format
/// and streaming capabilities, respectively.
pub trait Manifest: TomlableTrait + StreamableTrait {
    /// Validates the manifest.
    ///
    /// This method checks if the manifest meets certain criteria and returns a `Result`.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the manifest passes all validation checks.
    /// * `Err(Vec<String>)` if there are validation errors, with a vector containing
    ///   descriptions of each error.
    fn validate(&self) -> Result<(), Vec<String>>;

    /// Creates lock files associated with the manifest.
    ///
    /// This method is intended to generate a set of lock files. Each lock file is a TOML file
    /// that contains all the pieces of the various lock files, expanded and processed.
    /// This is what is streamed and used.
    ///
    /// # Returns
    /// A `HashMap<String, String>` where each key is a filename and each value is the TOML
    /// string content of the lock file.
    fn lock(&self) -> HashMap<String, String> {
        todo!("TODO: [manifests] Implement locking of manifests");
    }

    /// Saves this manifest to a specified file.
    ///
    /// This method is responsible for persisting the manifest data to a file at the given path.
    ///
    /// # Arguments
    /// * `path` - A `PathBuf` representing the file path where the manifest should be saved.
    ///
    /// # Returns
    /// * `Ok(())` if the save operation is successful.
    /// * `Err(())` if the save operation fails.
    fn save(&self, _path: PathBuf) -> Result<(), ()> {
        todo!("TODO: [manifests] Implement saving of manifests");
    }
}

/// Represents a vendor who creates a game, peripheral, playable, or other component.
#[derive(Debug,Serialize, Deserialize, PartialEq, Tomlable, Jsonable, Streamable, Getters, Setters, Clone)]
#[getset(get = "pub")]
pub struct Vendor {
    name: String,
    url: Url,
}

impl Display for Vendor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Vendor {
    pub fn new(name: &str, url: &str) -> Self {
        Vendor {
            name: name.into(),
            url: Url::parse(url).expect("Failed to parse URL"),
        }
    }
}

/// Represents modifiers that can be applied to a range.
///
/// Used to describe things like "there must be an even number of players"
#[derive(Tomlable, Jsonable, Streamable, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RangeModifier {
    Even,
    Odd,
}

/// Represents a range of numbers. Used to say things like "there must be between 2 and 4 players".
///
/// Ranges are inclusive, so 1-3 means 1, 2, and 3 are all valid.
// TODO: [manifests] Right now any range must be a string, but we should allow a single integer to work
#[derive(Tomlable, Jsonable, Streamable, Debug, Serialize, Getters, Setters, PartialEq, Eq)]
#[getset(get = "pub")]
pub struct Range {
    min: u8,
    max: u8,
    modifier: Option<RangeModifier>,
}

impl<'de> Deserialize<'de> for Range {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

impl FromStr for Range {
    type Err = &'static str; // TODO: [errors] This should be a real enum error

    fn from_str(range: &str) -> Result<Self, Self::Err> {
        let mut min: u8 = 0;
        let mut max: u8 = 0;
        let mut modifier: Option<RangeModifier> = None;

        let mut phase = RangeParsePhase::Min;
        let mut buffer = str!();

        let mut chars = range.chars().peekable();

        // TODO: [efficiency] This is a pretty inefficient way to parse a range. We should probably use a regex or something.
        while let Some(current_char) = chars.next() {
            let next_char = chars.peek();

            match phase {
                RangeParsePhase::Min => {
                    buffer.push(current_char);

                    match next_char {
                        Some(next) => {
                            if !next.is_numeric() {
                                min = buffer.parse().expect("Failed to parse min");
                                max = buffer.parse().expect("Failed to parse max");
                                buffer = str!();
                                phase = RangeParsePhase::Max;
                                continue;
                            }
                        }
                        None => {
                            min = buffer.parse().expect("Failed to parse min");
                            max = buffer.parse().expect("Failed to parse max");
                            buffer = str!();
                            phase = RangeParsePhase::Max;
                        }
                    }
                }
                RangeParsePhase::Max => {
                    if current_char == '+' {
                        max = 0;

                        match next_char {
                            Some(next) => {
                                if next == &'[' {
                                    phase = RangeParsePhase::Modifier;
                                    continue;
                                }
                            }
                            None => continue,
                        }
                    }

                    if current_char == '-' {
                        continue;
                    }

                    if current_char.is_numeric() {
                        buffer.push(current_char);
                    }

                    match next_char {
                        Some(next) => {
                            if next == &'[' {
                                max = buffer.parse().expect("Failed to parse max");
                                phase = RangeParsePhase::Modifier;
                                buffer = str!();
                            }
                        }
                        None => {
                            max = buffer.parse().expect("Failed to parse max");
                            phase = RangeParsePhase::Modifier;
                            buffer = str!();
                        }
                    }
                }
                RangeParsePhase::Modifier => {
                    if current_char == '[' {
                        continue;
                    }

                    // TODO: [implementation] this should check all the words
                    if current_char == 'e' {
                        modifier = Some(RangeModifier::Even);
                    }

                    if current_char == 'o' {
                        modifier = Some(RangeModifier::Odd);
                    }
                }
            }
        }

        Ok(Range { min, max, modifier })
    }
}

/// Represents the phase (or stage) of the range parsing.
///
/// See `Range::from_str()` for more information.
enum RangeParsePhase {
    Min,
    Max,
    Modifier,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::lib::{Range, RangeModifier};

    #[test]
    fn it_parses_a_single_number() {
        let str = "5";
        let range = Range::from_str(str).expect("Failed to parse range");

        assert_eq!(
            range,
            Range {
                min: 5,
                max: 5,
                modifier: None,
            }
        )
    }

    #[test]
    fn it_parses_a_single_long_number() {
        let str = "200";
        let range = Range::from_str(str).expect("Failed to parse range");

        assert_eq!(
            range,
            Range {
                min: 200,
                max: 200,
                modifier: None,
            }
        )
    }

    #[test]
    fn it_parses_a_min_number() {
        let str = "5+";
        let range = Range::from_str(str).expect("Failed to parse range");

        assert_eq!(
            range,
            Range {
                min: 5,
                max: 0,
                modifier: None,
            }
        )
    }

    #[test]
    fn it_parses_a_range_of_numbers() {
        let str = "5-10";
        let range = Range::from_str(str).expect("Failed to parse range");

        assert_eq!(
            range,
            Range {
                min: 5,
                max: 10,
                modifier: None,
            }
        )
    }

    #[test]
    fn it_parses_a_range_with_even() {
        let str = "5-10[even]";
        let range = Range::from_str(str).expect("Failed to parse range");

        assert_eq!(
            range,
            Range {
                min: 5,
                max: 10,
                modifier: Some(RangeModifier::Even),
            }
        )
    }

    #[test]
    fn it_parses_a_range_with_odd() {
        let str = "5-10[odd]";
        let range = Range::from_str(str).expect("Failed to parse range");

        assert_eq!(
            range,
            Range {
                min: 5,
                max: 10,
                modifier: Some(RangeModifier::Odd),
            }
        )
    }

    #[test]
    fn it_parses_a_min_with_odd() {
        let str = "5+[odd]";
        let range = Range::from_str(str).expect("Failed to parse range");

        assert_eq!(
            range,
            Range {
                min: 5,
                max: 0,
                modifier: Some(RangeModifier::Odd),
            }
        )
    }

    #[test]
    fn it_parses_a_min_with_even() {
        let str = "5+[even]";
        let range = Range::from_str(str).expect("Failed to parse range");

        assert_eq!(
            range,
            Range {
                min: 5,
                max: 0,
                modifier: Some(RangeModifier::Even),
            }
        )
    }
}
