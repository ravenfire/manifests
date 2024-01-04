use std::collections::HashMap;

use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use common::{
    data::ValidKey,
    macros::{Jsonable, Streamable, Tomlable},
    semver::Version,
    toml::Value,
    url::Url,
};

use crate::{LanguageMap, Vendor};

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
pub struct PeripheralManifest {
    /// The unique identifier for the peripheral
    /// `screen`
    name: ValidKey,

    /// The human readable name of the peripheral
    /// { en: "Screen", fr: "Écran" }
    titles: LanguageMap,

    /// Longer human readable description
    /// { en: "A screen", fr: "Un écran" }
    descriptions: LanguageMap,

    /// Semantic version of the hardware and software
    /// "1.23.01-alpha"
    version: Version,

    /// Vendor of the device
    /// "ravenfire"
    vendor: Vendor,

    /// UUID
    /// "a1b2c3d4-e5f6-g7h8-i9j0-k1l2m3n4o5p6"
    uuid: ValidKey,

    /// A list of specs it implements
    #[serde(default = "Vec::new")]
    provides: Vec<Provider>,

    /// A list of optional features it implements
    #[serde(default = "HashMap::new")]
    features: HashMap<ValidKey, Value>,
}

impl PeripheralManifest {
    pub fn id(&self) -> ValidKey {
        format!("{}/{}", self.vendor(), self.name())
            .try_into()
            .expect("Failed to create ValidKey")
    }
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
)]
#[getset(get = "pub", set = "pub")]
pub struct Provider {
    name: ValidKey,
    spec: Url,
    version: Version,
    #[serde(default = "default_count")]
    count: u32,
}

fn default_count() -> u32 {
    1
}

// TODO: [restructure] May not be the best place for this to live
#[derive(
    Tomlable, Jsonable, Streamable, Debug, Serialize, Deserialize, Getters, Setters, Clone,
)]
#[getset(get = "pub", set = "pub")]
pub struct Spec {
    key: ValidKey,
    url: Url,
    titles: LanguageMap,
    descriptions: LanguageMap,
    version: Version,
    features: Vec<Feature>,
}

#[derive(
    Tomlable, Jsonable, Streamable, Debug, Serialize, Deserialize, Getters, Setters, Clone,
)]
#[getset(get = "pub", set = "pub")]
pub struct Feature {
    key: ValidKey,
    titles: LanguageMap,
    descriptions: LanguageMap,
}

#[cfg(test)]
mod tests {
    // use common::traits::{Jsonable, Streamable, Tomlable};
    //
    // use super::PeripheralManifest;
    //
    // #[test]
    // fn test() {
    //     let m = PeripheralManifest::new();
    //
    //     let stream = m.to_stream().unwrap();
    //     // let stream = m.to_json().unwrap();
    //     // let stream = m.to_toml().unwrap();
    //
    //     let des = PeripheralManifest::from_stream(&mut &stream[..]).unwrap();
    //     // let des = PeripheralManifest::from_json(&stream).unwrap();
    //     // let des = PeripheralManifest::from_toml(&stream).unwrap();
    //
    //     println!("{:?}", des);
    // }
}
