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
