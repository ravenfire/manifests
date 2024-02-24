//! Meta data for the project
//! This is all deprecated
use getset::{Getters, Setters};
use serde::{Deserialize, Deserializer, Serialize};

use common::chrono::{DateTime, Utc};
use common::datetime::DateTimeParsing;
use common::macros::{Jsonable, Streamable, Tomlable};

#[derive(
    Tomlable,
    Jsonable,
    Streamable,
    Debug,
    Serialize,
    Deserialize,
    Getters,
    Setters,
    Clone,
    PartialEq,
)]
#[getset(get = "pub", set = "pub")]
pub struct Meta {
    #[serde(deserialize_with = "my_deserialize_date")]
    generated: DateTime<Utc>,
}

fn my_deserialize_date<'de, D>(des: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(des)?;
    DateTime::<Utc>::parse_with_inference(&s).map_err(serde::de::Error::custom)
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            generated: Utc::now()
                .format("%Y-%m-%d %H:%M")
                .to_string()
                .parse()
                .unwrap(),
        }
    }
}
