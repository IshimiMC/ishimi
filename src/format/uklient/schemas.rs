use super::validations::validate_uklient_fallback_mc_version;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub(crate) enum Provider {
    #[serde(rename = "modrinth")]
    Modrinth,
    #[serde(rename = "curseforge")]
    Curseforge,
}

#[derive(Validate, Deserialize)]
pub(crate) struct UklientFallback {
    #[validate(custom = "validate_uklient_fallback_mc_version")]
    minecraft_version: String,
    version_id: String,
}

#[derive(Deserialize)]
pub(crate) struct UklientMod {
    name: String,
    id: String,
    provider: Provider,
    dependencies: Option<Vec<String>>,
    fallback: Option<Vec<UklientFallback>>,
    config: Option<Vec<String>>,
    any_version: Option<bool>,
}

#[derive(Deserialize)]
#[serde(transparent)]
pub(crate) struct UklientModJSON {
    arr: Vec<UklientMod>,
}

#[derive(Deserialize)]
pub(crate) struct UklientPreset {
    name: String,
    display_name: String,
    mods: Vec<String>,
}

#[derive(Deserialize)]
#[serde(transparent)]
pub(crate) struct UklientPresetJSON {
    arr: Vec<UklientPreset>,
}
