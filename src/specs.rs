use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use common::data::schema::Schema;
use common::data::{LanguageMap, ValidKey};
use common::macros::{Jsonable, Streamable, Tomlable};
use common::semver::Version;
use common::url::Url;

use crate::VendorFull;

#[derive(
    Tomlable, Jsonable, Streamable, Debug, Serialize, Deserialize, Getters, Setters, Clone,
)]
pub enum Spec {
    Reference(SpecReference),
    Full(SpecFull),
}

#[derive(
    Tomlable, Jsonable, Streamable, Debug, Serialize, Deserialize, Getters, Setters, Clone,
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
pub struct SpecFull {
    url: Url,
    vendor: VendorFull,
    version: Version,
    titles: LanguageMap,
    descriptions: LanguageMap,
    features: Vec<Feature>,
    schema: Schema,
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
    schema: Option<Schema>, // Can be an empty schema, in which case it will be ignored.
}