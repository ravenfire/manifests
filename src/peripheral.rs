use std::collections::HashMap;

use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use common::data::LanguageMap;
use common::{
    data::ValidKey,
    macros::{Jsonable, Streamable, Tomlable},
    semver::Version,
    toml::Value,
};

use crate::specs::SpecReference;
use crate::{LanguageMap, VendorFull};

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
    vendor: VendorFull,

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
    // Peripheral defined group
    spec: SpecReference,
    // TODO: START HERE -- This needs to be an enum Spec::Reference|Definition
    #[serde(default = "default_count")]
    count: u8,
}

fn default_count() -> u8 {
    1
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
