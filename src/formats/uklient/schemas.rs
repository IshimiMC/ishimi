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
    pub minecraft_version: String,
    pub version_id: String,
}

#[derive(Deserialize)]
pub(crate) struct UklientMod {
    pub name: String,
    pub id: String,
    pub provider: Provider,
    pub dependencies: Option<Vec<String>>,
    pub fallback: Option<Vec<UklientFallback>>,
    pub config: Option<Vec<String>>,
    pub any_version: Option<bool>,
}

#[derive(Deserialize)]
#[serde(transparent)]
pub(crate) struct UklientModJSON {
    pub arr: Vec<UklientMod>,
}

#[derive(Deserialize)]
pub(crate) struct UklientPreset {
    pub name: String,
    pub display_name: String,
    pub mods: Vec<String>,
}

#[derive(Deserialize)]
#[serde(transparent)]
pub(crate) struct UklientPresetJSON {
    pub arr: Vec<UklientPreset>,
}
