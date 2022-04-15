use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IshimiFormat {
    config_version: u32,
    meta: IshimiFormatMeta,
    versions: IshimiFormatVersions,
    mods: Option<Vec<IshimiFormatMod>>,
    files: Option<Vec<IshimiFormatFiles>>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IshimiFormatSide {
    #[serde(alias = "client")]
    Client,
    #[serde(alias = "server")]
    Server,
    #[serde(alias = "both")]
    Both
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IshimiFormatMeta {
    name: String,
    author: String,
    version: String,
    side: IshimiFormatSide
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IshimiFormatVersions {
    minecraft: String,
    fabric: Option<String>,
    forge: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IshimiFormatMod {
    name: String,
    friendly_name: Option<String>,
    filename: Option<String>,
    version: Option<String>,
    direct: Option<Vec<IshimiFormatModDirect>>,
    modrinth: Option<Vec<IshimiFormatModProvider>>,
    curseforge: Option<Vec<IshimiFormatModProvider>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IshimiFormatModDirect {
    url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IshimiFormatModProvider {
    #[serde(alias = "mod_id")]
    project_id: String,
    #[serde(alias = "version")]
    file_id: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IshimiFormatFiles {
    url: String
}