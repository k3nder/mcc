use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::string::String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profiles {
    pub profiles: HashMap<String, Profile>,
    #[serde(skip)]
    settings: (),
    #[serde(skip)]
    version: f32,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Profile {
    pub created: String,
    pub icon: Option<String>,
    pub lastUsed: String,
    pub lastVersionId: String,
    pub name: String,
    pub javaArgs: Option<String>,
    pub javaDir: Option<String>,
    pub gameDir: Option<String>,
    pub resolution: Option<Resolution>,
    #[serde(rename = "type")]
    pub typ: String,
}

impl Profile {
    pub fn set_name(&mut self, str: String) {
        self.name = str;
    }
    pub fn set_version_id(&mut self, str: String) {
        self.lastVersionId = str;
    }
    pub fn set_game_dir(&mut self, str: Option<String>) {
        self.gameDir = str;
    }
    pub fn set_java_dir(&mut self, str: Option<String>) {
        self.javaDir = str;
    }
    pub fn set_java_args(&mut self, str: Option<String>) {
        self.javaArgs = str;
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Resolution {
    height: u32,
    width: u32,
}
impl Profiles {
    pub fn deserialize(&self) -> String {
        serde_json::to_string(self).expect("error in deserialize")
    }
    pub fn serialize(str_: &str) -> Profiles {
        let mut file = File::open(str_).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        serde_json::from_str(&*content).expect("error in serialize")
    }
    pub fn add(&mut self, profile: Profile, key: &str) {
        self.profiles.insert(key.parse().unwrap(), profile);
    }
    pub fn remove(&mut self, key: &str) {
        self.profiles.remove(key);
    }
    pub fn save(&self, file_str: &str) {
        let str_aux = self.deserialize();
        let mut file = File::open(file_str).unwrap();
        println!("{}", str_aux);
        fs::write(Path::new(file_str), str_aux).expect("Error in save, profile_json.rs");
        //file.write_all(str_aux.as_bytes()).expect("Error in save, profile_json.rs");
    }
}
