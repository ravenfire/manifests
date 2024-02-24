use std::fmt::{Display, Formatter};

use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use common::data::key::ValidKey;
use common::data::LanguageMap;
use common::email_address::EmailAddress;
use common::macros::{Jsonable, Streamable, Tomlable};
use common::url::Url;

/// Represents a vendor who creates a game, peripheral, playable, or other component.
#[derive(
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Tomlable,
    Jsonable,
    Streamable,
    Getters,
    Setters,
    Clone,
    Builder,
    Default,
)]
#[getset(get = "pub")]
pub struct Vendor {
    name: ValidKey,
    #[builder(default)]
    #[serde(default)]
    titles: LanguageMap,
    #[builder(default)]
    #[serde(default)]
    descriptions: LanguageMap,
    #[builder(setter(into, strip_option), default)]
    url: Option<Url>,
    #[builder(setter(into, strip_option), default)]
    email: Option<EmailAddress>,
    #[builder(setter(into, strip_option), default)]
    support: Option<Url>,
}

impl Display for Vendor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Vendor {
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.try_into().unwrap(),
            titles: Default::default(),
            descriptions: Default::default(),
            url: Some(Url::parse(url).expect("Failed to parse URL")),
            email: None,
            support: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use maplit::hashmap;

    use common::data::serialization::Jsonable;
    use common::email_address::EmailAddress;
    use common::isolang::Language;
    use common::url::Url;

    use crate::vendor::{Vendor, VendorBuilder};

    #[test]
    fn it_serializes_basic_vendor() {
        let vendor = VendorBuilder::default()
            .name("ravenfire".try_into().unwrap())
            .build()
            .unwrap();

        let serialized = vendor.to_json().unwrap();
        let json = r#"{"name":"ravenfire","titles":{},"descriptions":{},"url":null,"email":null,"support":null}"#;
        assert_eq!(serialized, json);

        let deserialized = Vendor::from_json(&json).unwrap();
        assert_eq!(deserialized, vendor);

        // Will panic if it fails
        let example = crate::examples::Vendor::ravenfire_min();
        let _ = example.build();
    }

    #[test]
    fn it_serializes_full_vendor() {
        let vendor = VendorBuilder::default()
            .name("ravenfire".try_into().unwrap())
            .titles(hashmap! {
                Language::from_str("en").unwrap() => "Raven Fire".to_owned(),
            })
            .descriptions(hashmap! {
                Language::from_str("en").unwrap() => "Raven Fire Games".to_owned(),
            })
            .url(Url::parse("https://ravenfire.games").unwrap())
            .support(Url::parse("https://ravenfire.games/support").unwrap())
            .email(EmailAddress::from_str("support@ravenfire.games").unwrap())
            .build()
            .unwrap();

        let serialized = vendor.to_json().unwrap();
        let json = r#"{"name":"ravenfire","titles":{"eng":"Raven Fire"},"descriptions":{"eng":"Raven Fire Games"},"url":"https://ravenfire.games/","email":"support@ravenfire.games","support":"https://ravenfire.games/support"}"#;
        assert_eq!(serialized, json);

        let deserialized = Vendor::from_json(&json).unwrap();
        assert_eq!(deserialized, vendor);

        // Will panic if it fails
        let example = crate::examples::Vendor::ravenfire();
        let _ = example.build();
    }
}
