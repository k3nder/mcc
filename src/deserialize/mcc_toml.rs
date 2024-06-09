use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct MccToml {
    #[serde(rename = "javaHomes")]
    pub java_homes: String,
    #[serde(rename = "versions")]
    pub versions: String,
    #[serde(rename = "profilesFile")]
    pub profiles_file: String,
    #[serde(rename = "manifestURL")]
    pub manifest_url: String,
    #[serde(rename = "assetsURL")]
    pub assets_url: String,
    #[serde(rename = "versionsBin")]
    pub versions_bin: String,
    #[serde(rename = "assetsDir")]
    pub assets_dir: String,
    #[serde(rename = "libsDir")]
    pub libs_dir: String,
    #[serde(rename = "versionsJsonDir")]
    pub versions_json_dir: String,
    #[serde(rename = "defaultGameDir")]
    pub default_game_dir: String,
}

impl MccToml {
    pub fn deserialize(&self) -> String {
        toml::to_string(self).expect("error in deserialize")
    }
    pub fn serialize(str_: &str) -> MccToml {
        toml::from_str(str_).expect("error in serialize")
    }
}
