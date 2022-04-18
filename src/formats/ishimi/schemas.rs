use semver::Op;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IshimiFormat {
    pub config_version: u32,
    pub meta: IshimiFormatMeta,
    pub versions: IshimiFormatVersions,
    pub mods: Option<Vec<IshimiFormatMod>>,
    pub files: Option<Vec<IshimiFormatFiles>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IshimiFormatSide {
    #[serde(alias = "client")]
    Client,
    #[serde(alias = "server")]
    Server,
    #[serde(alias = "both")]
    Both,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IshimiFormatMeta {
    pub name: String,
    pub author: String,
    pub version: String,
    pub side: IshimiFormatSide,
    pub base: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IshimiFormatVersions {
    pub minecraft: String,
    pub fabric: Option<String>,
    pub forge: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IshimiFormatMod {
    pub name: String,
    pub friendly_name: Option<String>,
    pub filename: Option<String>,
    pub version: Option<String>,
    pub direct: Option<Vec<IshimiFormatModDirect>>,
    pub modrinth: Option<Vec<IshimiFormatModProvider>>,
    pub curseforge: Option<Vec<IshimiFormatModProvider>>,
    pub github: Option<Vec<IshimiFormatModGithub>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IshimiFormatModDirect {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IshimiFormatModProvider {
    #[serde(alias = "slug")]
    pub name: Option<String>,
    #[serde(alias = "mod_id")]
    pub project_id: Option<String>,
    #[serde(alias = "version")]
    pub file_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IshimiFormatModGithub {
    #[serde(alias = "user")]
    pub owner: String,
    #[serde(alias = "repository")]
    pub repo: String,
    #[serde(alias = "release")]
    pub tag: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IshimiFormatFiles {
    pub url: String,
    #[serde(alias = "dest")]
    pub destination: Option<String>
}
