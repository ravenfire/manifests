use std::collections::HashMap;

use getset::{Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

use common::data::ValidKey;
use common::macros::{Jsonable, Streamable, Tomlable};
use common::manifests::LanguageMap;
use common::manifests::{Range, Vendor};
use common::semver::{Version, VersionReq};
use common::url::Url;

#[derive(
    Tomlable, Jsonable, Streamable, Debug, Serialize, Deserialize, Getters, Setters, MutGetters,
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
    Tomlable, Jsonable, Streamable, Debug, Serialize, Deserialize, Getters, Setters, MutGetters,
)]
#[getset(get = "pub", set = "pub")]
pub struct Scenario {
    name: ValidKey,
    titles: LanguageMap,
    descriptions: LanguageMap,
    players: Vec<ScenarioPlayer>,
}

#[derive(
    Tomlable, Jsonable, Streamable, Debug, Serialize, Deserialize, Getters, Setters, MutGetters,
)]
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
    Tomlable, Jsonable, Streamable, Debug, Serialize, Deserialize, Getters, Setters, MutGetters,
)]
#[getset(get = "pub", set = "pub")]
pub struct Requirement {
    name: ValidKey,
    spec: Url,
    version: VersionReq,
    count: Range,
    #[serde(default = "bool::default")]
    optional: bool,
    #[serde(default = "HashMap::default")]
    features: HashMap<ValidKey, FeatureValue>,
    definition: Option<String>, // @todo: this will need to be much more robust
}

#[derive(Tomlable, Jsonable, Streamable, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", from = "String")]
pub enum FeatureValue {
    Required,
    Optional,
    Value(String), // TODO: [manifests, implementation] this should probably support payload values instead of just a string
}

impl From<String> for FeatureValue {
    fn from(s: String) -> Self {
        return match s.as_str() {
            "required" => Self::Required,
            "optional" => Self::Optional,
            _ => Self::Value(s),
        };
    }
}
