use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use common::data::key::ValidKey;
use common::data::LanguageMap;
use common::url::Url;
use common::{
    macros::{Jsonable, Streamable, Tomlable},
    semver::Version,
};

use crate::specs::Spec;
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
    Clone,
    PartialEq,
    Builder,
)]
#[getset(get = "pub", set = "pub")]
pub struct PeripheralManifest {
    /// Semantic version of the hardware and software
    /// "1.23.01-alpha"
    version: Version,

    /// UUID
    /// "a1b2c3d4-e5f6-g7h8-i9j0-k1l2m3n4o5p6"
    uuid: ValidKey,

    /// Vendor of the device
    vendor: Vendor,

    /// The human readable name of the peripheral
    /// { en: "Screen", fr: "Écran" }
    #[serde(default)]
    #[builder(default)]
    titles: LanguageMap,

    /// Longer human readable description
    /// { en: "A screen", fr: "Un écran" }
    #[serde(default)]
    #[builder(default)]
    descriptions: LanguageMap,

    #[serde(default)]
    #[builder(default)]
    url: Option<Url>,

    #[serde(default)]
    #[builder(default)]
    support: Option<Url>,

    /// A list of specs it implements
    provides: Vec<Provider>,
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
    Clone,
    PartialEq,
    Builder,
)]
#[getset(get = "pub", set = "pub")]
pub struct Provider {
    // Peripheral defined group
    name: ValidKey,
    spec: Spec,
    count: u8,
}

#[cfg(test)]
mod tests {
    use common::data::serialization::Jsonable;

    use crate::peripheral::PeripheralManifest;

    #[test]
    fn it_serializes_peripheral() {
        // We start with a Game
        let example = crate::examples::Peripheral::watertribe_card_reader();

        // Let's build a manifest
        let manifest = example.build();

        // And let's do it through a round trip
        let serialized = manifest.to_json().unwrap();
        let deserialized = PeripheralManifest::from_json(&serialized).unwrap();

        assert_eq!(deserialized, manifest);
    }
}
