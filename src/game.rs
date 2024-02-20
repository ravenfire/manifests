use getset::{Getters, MutGetters, Setters};

use common::data::{LanguageMap, ValidKey};
use common::macros::{Jsonable, Streamable, Tomlable};
use common::semver::{Version, VersionReq};
use common::serde::{Deserialize, Serialize};
use common::url::Url;

use crate::range::Range;
use crate::Vendor;

#[derive(
    Tomlable,
    Jsonable,
    Streamable,
    Debug,
    Serialize,
    Deserialize,
    Getters,
    Setters,
    MutGetters,
    Clone,
)]
#[getset(get = "pub", set = "pub")]
pub struct GameManifest {
    name: ValidKey,
    titles: LanguageMap,
    descriptions: LanguageMap,
    vendor: Vendor,
    version: Version,
    url: Url,
    scenarios: Vec<Scenario>,
}

#[derive(
    Tomlable,
    Jsonable,
    Streamable,
    Debug,
    Serialize,
    Deserialize,
    Getters,
    Setters,
    MutGetters,
    Clone,
)]
#[getset(get = "pub", set = "pub")]
pub struct Scenario {
    name: ValidKey,
    titles: LanguageMap,
    descriptions: LanguageMap,
    players: Vec<ScenarioPlayer>,
}

#[derive(
    Tomlable,
    Jsonable,
    Streamable,
    Debug,
    Serialize,
    Deserialize,
    Getters,
    Setters,
    MutGetters,
    Clone,
)]
// Analogous to PlayerType
#[getset(get = "pub", set = "pub")]
pub struct ScenarioPlayer {
    name: ValidKey,
    titles: LanguageMap,
    descriptions: LanguageMap,
    count: Range,
    #[serde(default = "Vec::default")]
    io: Vec<Requirement>,
    #[serde(default = "Vec::default")]
    playables: Vec<Requirement>,
}

#[derive(
    Tomlable,
    Jsonable,
    Streamable,
    Debug,
    Serialize,
    Deserialize,
    Getters,
    Setters,
    MutGetters,
    Clone,
)]
#[getset(get = "pub", set = "pub")]
pub struct Requirement {
    name: ValidKey,
    spec: Url,
    version: VersionReq,
    count: Range,
    // #[serde(default = "bool::default")]
    // optional: bool,
    #[serde(default = "Vec::default")]
    features: Vec<ValidKey>,
}

// features = ["nfc", "facing", "indicators(3)"]
