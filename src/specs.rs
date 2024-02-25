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
