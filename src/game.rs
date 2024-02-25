use getset::{Getters, MutGetters, Setters};

use common::data::key::ValidKey;
use common::data::LanguageMap;
use common::macros::{Jsonable, Streamable, Tomlable};
use common::semver::{Version, VersionReq};
use common::serde::{Deserialize, Serialize};
use common::url::Url;

use crate::range::Range;
use crate::vendor::Vendor;

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
    PartialEq,
    Builder,
)]
#[getset(get = "pub", set = "pub")]
pub struct GameManifest {
    #[serde(default)]
    #[builder(default)]
    titles: LanguageMap,
    #[serde(default)]
    #[builder(default)]
    descriptions: LanguageMap,
    version: Version,
    url: Option<Url>,
    support: Option<Url>,
    vendor: Vendor,
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
    PartialEq,
    Builder,
)]
#[getset(get = "pub", set = "pub")]
pub struct Scenario {
    name: ValidKey,
    #[serde(default)]
    #[builder(default)]
    titles: LanguageMap,
    #[serde(default)]
    #[builder(default)]
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
    PartialEq,
    Builder,
)]
// Analogous to PlayerType
#[getset(get = "pub", set = "pub")]
pub struct ScenarioPlayer {
    name: ValidKey,
    #[serde(default)]
    #[builder(default)]
    titles: LanguageMap,
    #[serde(default)]
    #[builder(default)]
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
    PartialEq,
    Builder,
)]
#[getset(get = "pub", set = "pub")]
pub struct Requirement {
    // Game Defined Group
    name: ValidKey,
    spec: Url,
    version: VersionReq,
    count: u32,
    #[serde(default = "Vec::default")]
    features: Vec<ValidKey>,
}

#[cfg(test)]
mod tests {
    use common::data::serialization::Jsonable;

    use crate::examples::Example;
    use crate::game::GameManifest;

    #[test]
    fn it_serializes_game() {
        // We start with a Game
        let example = crate::examples::games::Game::simple_battle();

        // Let's build a manifest
        let manifest = example.build();

        // And let's do it through a round trip
        let serialized = manifest.to_json().unwrap();
        let deserialized = GameManifest::from_json(&serialized).unwrap();

        assert_eq!(deserialized, manifest);
    }
}
