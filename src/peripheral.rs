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
    use std::str::FromStr;

    use maplit::hashmap;

    use common::data::serialization::Jsonable;
    use common::isolang::Language;
    use common::semver::Version;
    use common::url::Url;

    use crate::peripheral::{PeripheralManifest, PeripheralManifestBuilder};

    #[test]
    fn it_serializes_peripheral() {
        let peripheral = PeripheralManifestBuilder::default()
            .version(Version::from_str("1.0.0").unwrap())
            .uuid("a1b2c3d4-e5f6-g7h8-i9j0-k1l2m3n4o5p6".try_into().unwrap())
            .vendor(crate::examples::Vendor::ravenfire_min().build())
            .url(Some(
                Url::parse("https://ravenfire.games/peripherals/card_reader").unwrap(),
            ))
            .titles(hashmap! {
                Language::from_str("en").unwrap() => "Card Reader".to_owned(),
            })
            .provides(vec![
                // ProviderBuilder::default()
                //     .name("peripheral_defined_group_card_reader".try_into().unwrap())
                //     .spec(crate::examples::Spec::card().build())
                //     .count(5)
                //     .build()
                //     .unwrap()
            ])
            // Descriptions and support left out on purpose to make sure it works with those optional fields
            .build()
            .unwrap();

        let serialized = peripheral.to_json().unwrap();

        let deserialized = PeripheralManifest::from_json(&serialized).unwrap();
        assert_eq!(deserialized, peripheral);

        // Will panic if it fails
        let example = crate::examples::Peripheral::watertribe_card_reader();
        example.build();
    }
}
