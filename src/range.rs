use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use getset::{Getters, Setters};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use common::macros::{Jsonable, Streamable, Tomlable};
use common::str;

// use serde::de::Visitor;

/// Represents modifiers that can be applied to a range.
///
/// Used to describe things like "there must be an even number of players"
#[derive(Tomlable, Jsonable, Streamable, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum RangeModifier {
    Even,
    Odd,
}

impl Display for RangeModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            RangeModifier::Even => write!(f, "even"),
            RangeModifier::Odd => write!(f, "odd"),
        }
    }
}

/// Represents a range of numbers. Used to say things like "there must be between 2 and 4 players".
///
/// Ranges are inclusive, so 1-3 means 1, 2, and 3 are all valid.
// TODO: [manifests] Right now any range must be a string, but we should allow a single integer to work
#[derive(Tomlable, Jsonable, Streamable, Debug, Getters, Setters, PartialEq, Eq, Clone)]
#[getset(get = "pub")]
pub struct Range {
    pub min: u8,
    pub max: u8,
    modifier: Option<RangeModifier>,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut range = str!();
        if self.min == self.max {
            range.push_str(&self.min.to_string());
        } else if self.max == 0 {
            range.push_str(&self.min.to_string());
            range.push_str("+");
        } else {
            range.push_str(&self.min.to_string());
            range.push_str("-");
            range.push_str(&self.max.to_string());
        }

        match &self.modifier {
            Some(modifier) => {
                range.push_str("[");
                range.push_str(&modifier.to_string());
                range.push_str("]");
            }
            None => {}
        }

        write!(f, "{}", range)
    }
}

impl Serialize for Range {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Range {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RangeVisitor;

        impl<'de> de::Visitor<'de> for RangeVisitor {
            type Value = Range;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or an integer representing a range")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Range, E>
            where
                E: de::Error,
            {
                Ok(Range {
                    min: value as u8,
                    max: value as u8,
                    modifier: None,
                })
            }

            fn visit_str<E>(self, value: &str) -> Result<Range, E>
            where
                E: de::Error,
            {
                // Assuming you have a parse function for Range
                value.parse::<Range>().map_err(E::custom)
            }
        }

        deserializer.deserialize_any(RangeVisitor)
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
    use serde::{Deserialize, Serialize};

    use common::macros::Tomlable;

    use crate::range::Range;

    #[derive(Serialize, Deserialize, Tomlable, PartialEq, Debug)]
    struct Container {
        range: Range,
    }

    mod deserialize {
        use common::data::serialization::Tomlable;

        use crate::range::tests::Container;
        use crate::range::{Range, RangeModifier};

        #[test]
        fn deserialize_an_exact_count_as_string() {
            let expected = Container {
                range: Range {
                    min: 1,
                    max: 1,
                    modifier: None,
                },
            };

            let toml = "range = \"1\"\n";
            let container = Container::from_toml(toml).expect("Failed to deserialize from TOML");
            assert_eq!(container, expected);
        }

        #[test]
        fn deserialize_an_exact_count_as_integer() {
            let expected = Container {
                range: Range {
                    min: 7,
                    max: 7,
                    modifier: None,
                },
            };

            let toml = "range = 7\n";
            let container = Container::from_toml(toml).expect("Failed to deserialize from TOML");
            assert_eq!(container, expected);
        }

        #[test]
        fn deserialize_a_min_and_max() {
            let expected = Container {
                range: Range {
                    min: 1,
                    max: 3,
                    modifier: None,
                },
            };

            let toml = "range = \"1-3\"\n";
            let container = Container::from_toml(toml).expect("Failed to deserialize from TOML");
            assert_eq!(container, expected);
        }

        #[test]
        fn deserialize_with_no_max() {
            let expected = Container {
                range: Range {
                    min: 1,
                    max: 0,
                    modifier: None,
                },
            };

            let toml = "range = \"1+\"\n";
            let container = Container::from_toml(toml).expect("Failed to deserialize from TOML");
            assert_eq!(container, expected);
        }

        #[test]
        fn deserialize_a_range_with_even() {
            let expected = Container {
                range: Range {
                    min: 1,
                    max: 4,
                    modifier: Some(RangeModifier::Even),
                },
            };

            let toml = "range = \"1-4[even]\"\n";
            let container = Container::from_toml(toml).expect("Failed to deserialize from TOML");
            assert_eq!(container, expected);
        }

        #[test]
        fn deserialize_a_range_with_odd() {
            let expected = Container {
                range: Range {
                    min: 1,
                    max: 4,
                    modifier: Some(RangeModifier::Odd),
                },
            };

            let toml = "range = \"1-4[odd]\"\n";
            let container = Container::from_toml(toml).expect("Failed to deserialize from TOML");
            assert_eq!(container, expected);
        }

        #[test]
        fn deserialize_a_min_with_even() {
            let expected = Container {
                range: Range {
                    min: 2,
                    max: 0,
                    modifier: Some(RangeModifier::Even),
                },
            };

            let toml = "range = \"2+[even]\"\n";
            let container = Container::from_toml(toml).expect("Failed to deserialize from TOML");
            assert_eq!(container, expected);
        }
    }

    mod serialize {
        use common::data::serialization::Tomlable;

        use crate::range::tests::Container;
        use crate::range::{Range, RangeModifier};

        #[test]
        fn serialize_an_exact_count() {
            let container = Container {
                range: Range {
                    min: 1,
                    max: 1,
                    modifier: None,
                },
            };

            let toml = container.to_toml().expect("Failed to serialize to TOML");
            assert_eq!(toml, "range = \"1\"\n");
        }

        #[test]
        fn serialize_a_min_and_max() {
            let container = Container {
                range: Range {
                    min: 1,
                    max: 3,
                    modifier: None,
                },
            };

            let toml = container.to_toml().expect("Failed to serialize to TOML");
            assert_eq!(toml, "range = \"1-3\"\n");
        }

        #[test]
        fn serialize_a_min_with_no_max() {
            let container = Container {
                range: Range {
                    min: 1,
                    max: 0,
                    modifier: None,
                },
            };

            let toml = container.to_toml().expect("Failed to serialize to TOML");
            assert_eq!(toml, "range = \"1+\"\n");
        }

        #[test]
        fn serialize_a_range_with_even() {
            let container = Container {
                range: Range {
                    min: 1,
                    max: 4,
                    modifier: Some(RangeModifier::Even),
                },
            };

            let toml = container.to_toml().expect("Failed to serialize to TOML");
            assert_eq!(toml, "range = \"1-4[even]\"\n");
        }

        #[test]
        fn serialize_a_range_with_odd() {
            let container = Container {
                range: Range {
                    min: 1,
                    max: 5,
                    modifier: Some(RangeModifier::Odd),
                },
            };

            let toml = container.to_toml().expect("Failed to serialize to TOML");
            assert_eq!(toml, "range = \"1-5[odd]\"\n");
        }

        #[test]
        fn serialize_a_min_with_odd() {
            let container = Container {
                range: Range {
                    min: 1,
                    max: 0,
                    modifier: Some(RangeModifier::Odd),
                },
            };

            let toml = container.to_toml().expect("Failed to serialize to TOML");
            assert_eq!(toml, "range = \"1+[odd]\"\n");
        }
    }

    /// Test From String
    mod from_str {
        use std::str::FromStr;

        use crate::range::{Range, RangeModifier};

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
}
