use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use common::data::schema::Property;
use common::data::{LanguageMap, ValidKey};
use common::macros::{Jsonable, Streamable, Tomlable};
use common::semver::Version;
use common::url::Url;

use crate::meta::Meta;
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
)]
#[getset(get = "pub", set = "pub")]
pub struct SpecReference {
    url: Url,
    version: Version,
    features: Vec<ValidKey>,
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
    Builder,
    PartialEq,
)]
#[getset(get = "pub", set = "pub")]
pub struct Spec {
    #[builder(default)]
    #[serde(default)]
    meta: Meta,
    version: Version,
    url: Url,
    vendor: Vendor,
    #[builder(default)]
    #[serde(default)]
    titles: LanguageMap,
    #[builder(default)]
    #[serde(default)]
    descriptions: LanguageMap,
    #[builder(default)]
    #[serde(default)]
    properties: Vec<Property>,
    #[builder(default)]
    #[serde(default)]
    features: Vec<Feature>,
}

/// Features are purely additive. They are used to add additional functionality to a spec.
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
    Builder,
    Default,
    PartialEq,
)]
#[getset(get = "pub", set = "pub")]
pub struct Feature {
    key: ValidKey,
    #[serde(default)]
    titles: LanguageMap,
    #[serde(default)]
    descriptions: LanguageMap,
    #[serde(default)]
    properties: Vec<Property>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_serializes_min_spec() {
        // let meta = Meta::new(DateTime::parse_with_inference("2024-02-24 16:18").unwrap());
        //
        // let spec = SpecBuilder::default()
        //     .version(Version::from_str("1.0.0").unwrap())
        //     .meta(meta)
        //     .url(Url::parse("https://ravenfire.games/specs/spec.json").unwrap())
        //     .vendor(crate::examples::Vendor::ravenfire_min().build())
        //     .build()
        //     .unwrap();
        //
        // let serialized = spec.to_json().unwrap();
        // let json = r#"{"meta":{"generated":"2024-02-24T16:18:00Z"},"version":"1.0.0","url":"https://ravenfire.games/specs/spec.json","vendor":{"name":"ravenfire","titles":{},"descriptions":{},"url":null,"email":null,"support":null},"titles":{},"descriptions":{},"properties":[],"features":[]}"#;
        // assert_eq!(serialized, json);

        // let deserialized = Spec::from_json(&json).unwrap();
        // assert_eq!(deserialized, spec);

        // Will panic if it fails
        let example = crate::examples::Spec::card();
        let built = example.build();
    }

    // #[test]
    // fn it_serializes_full_spec() {
    //     let spec = VendorBuilder::default()
    //         .name("ravenfire".try_into().unwrap())
    //         .titles(hashmap! {
    //             Language::from_str("en").unwrap() => "Raven Fire".to_owned(),
    //         })
    //         .descriptions(hashmap! {
    //             Language::from_str("en").unwrap() => "Raven Fire Games".to_owned(),
    //         })
    //         .url(Url::parse("https://ravenfire.games").unwrap())
    //         .support(Url::parse("https://ravenfire.games/support").unwrap())
    //         .email(EmailAddress::from_str("support@ravenfire.games").unwrap())
    //         .build()
    //         .unwrap();
    //
    //     let serialized = spec.to_json().unwrap();
    //     let json = r#"{"name":"ravenfire","titles":{"eng":"Raven Fire"},"descriptions":{"eng":"Raven Fire Games"},"url":"https://ravenfire.games/","email":"support@ravenfire.games","support":"https://ravenfire.games/support"}"#;
    //     assert_eq!(serialized, json);
    //
    //     let deserialized = Vendor::from_json(&json).unwrap();
    //     assert_eq!(deserialized, spec);
    //
    //     // Will panic if it fails
    //     let example = crate::examples::Vendor::ravenfire();
    //     let built = example.build();
    // }
}
