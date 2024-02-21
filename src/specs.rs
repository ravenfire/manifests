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
    Tomlable, Jsonable, Streamable, Debug, Serialize, Deserialize, Getters, Setters, Clone,
)]
#[getset(get = "pub", set = "pub")]
pub struct Spec {
    meta: Meta,
    version: Version,
    url: Url,
    vendor: Vendor,
    titles: LanguageMap,
    descriptions: LanguageMap,
    properties: Vec<Property>,
    features: Vec<Feature>,
}

/// Features are purely additive. They are used to add additional functionality to a spec.
#[derive(
    Tomlable, Jsonable, Streamable, Debug, Serialize, Deserialize, Getters, Setters, Clone,
)]
#[getset(get = "pub", set = "pub")]
pub struct Feature {
    key: ValidKey,
    titles: LanguageMap,
    descriptions: LanguageMap,
    properties: Vec<Property>,
}
