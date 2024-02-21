use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use common::chrono::{DateTime, Utc};
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
    generated: DateTime<Utc>,
}
