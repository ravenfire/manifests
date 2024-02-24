use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use common::data::key::ValidKey;
use common::data::schema::Property;
use common::data::LanguageMap;
use common::macros::{Jsonable, Streamable, Tomlable};
use common::semver::Version;
use common::url::Url;

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
    // TODO [question] I don't think I actually need to capture the meta here.
    // #[builder(default)]
    // #[serde(default)]
    // meta: Meta,
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
    use std::str::FromStr;

    use maplit::hashmap;

    use common::data::serialization::Jsonable;
    use common::isolang::Language;
    use common::semver::Version;
    use common::url::Url;

    use crate::specs::{Spec, SpecBuilder};

    #[test]
    fn it_serializes_min_spec() {
        let spec = SpecBuilder::default()
            .version(Version::from_str("1.0.0").unwrap())
            .url(Url::parse("https://ravenfire.games/specs/spec.json").unwrap())
            .vendor(crate::examples::Vendor::ravenfire_min().build())
            .build()
            .unwrap();

        let serialized = spec.to_json().unwrap();
        let json = r#"{"version":"1.0.0","url":"https://ravenfire.games/specs/spec.json","vendor":{"name":"ravenfire","titles":{},"descriptions":{},"url":null,"email":null,"support":null},"titles":{},"descriptions":{},"properties":[],"features":[]}"#;
        assert_eq!(serialized, json);

        let min_json = r#"{"version":"1.0.0","url":"https://ravenfire.games/specs/spec.json","vendor":{"name":"ravenfire"}}"#;
        let deserialized = Spec::from_json(&min_json).unwrap();
        assert_eq!(deserialized, spec);

        // Will panic if it fails
        let example = crate::examples::Spec::card();
        let _ = example.build();
    }

    #[test]
    fn it_serializes_full_spec() {
        let spec = SpecBuilder::default()
            .version(Version::from_str("1.0.0").unwrap())
            .url(Url::parse("https://ravenfire.games/specs/spec.json").unwrap())
            .vendor(crate::examples::Vendor::ravenfire().build())
            .titles(hashmap! {
                Language::from_str("en").unwrap() => "Raven Fire".to_owned(),
            })
            .descriptions(hashmap! {
                Language::from_str("en").unwrap() => "Raven Fire Games".to_owned(),
            })
            .properties(vec![
                crate::examples::Property::facing().build(),
                crate::examples::Property::weapon().build(),
            ])
            .features(vec![
                crate::examples::Feature::led().build(),
                crate::examples::Feature::rfid().build(),
            ])
            .build()
            .unwrap();

        let serialized = spec.to_json().unwrap();
        let json = r#"{"version":"1.0.0","url":"https://ravenfire.games/specs/spec.json","vendor":{"name":"ravenfire","titles":{"eng":"Raven Fire"},"descriptions":{"eng":"Raven Fire"},"url":"https://ravenfire.games/","email":"vendor@ravenfire.games","support":"https://ravenfire.games/dev/support"},"titles":{"eng":"Raven Fire"},"descriptions":{"eng":"Raven Fire Games"},"properties":[{"key":"facing","data_type":"String","titles":{},"descriptions":{},"optional":false,"enumerations":[{"String":"up"},{"String":"down"}]},{"key":"weapon","data_type":"Playable","titles":{"eng":"Playable"},"descriptions":{"eng":"Which weapon do you have?"},"optional":true,"enumerations":[]}],"features":[{"key":"led","titles":{"eng":"LED"},"descriptions":{"eng":"Is an LED Light On?"},"properties":[{"key":"color","data_type":"String","titles":{"eng":"Color"},"descriptions":{"eng":"What color is the LED?"},"optional":false,"enumerations":[{"String":"red"},{"String":"green"},{"String":"blue"}]}]},{"key":"rfid","titles":{},"descriptions":{},"properties":[]}]}"#;
        assert_eq!(serialized, json);

        // Will panic if it fails
        let example = crate::examples::Spec::card();
        let _ = example.build();
    }
}
