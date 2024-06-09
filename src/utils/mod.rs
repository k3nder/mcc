pub mod console_log;

use config::{Config, ConfigBuilder, File, FileFormat};
use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::path::Path;

pub fn replace_map(str: &str, map: HashMap<String, String>) -> String {
    let mut result = str.to_string();
    for (key, value) in map {
        result = result.replace(format!("[{}]", key).as_str(), value.as_str())
    }
    result
}
pub fn replace_mcc_config(str: &str) -> String {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(
        "env.dir".to_string(),
        "app".to_string(),
    );
    map.insert(
        "appdata.roaming".to_string(),
        format!("C:\\Users\\{}\\AppData\\Roaming", get_user_name()),
    );
    replace_map(str, map)
}

pub fn settings(key: &str) -> String {
    let mut settings = Config::builder().add_source(File::new(".ini", FileFormat::Ini));
    let config = settings.build().unwrap();
    replace_mcc_config(
        config
            .get_string(key)
            .expect(format!("cannot found key: {} in settings", key).as_str())
            .as_str(),
    )
}
pub fn settings_set(key: &str, value: &str) {
    let mut settings = Config::builder().add_source(File::new(".ini", FileFormat::Ini));
    let mut config = settings.build().unwrap();
    config.set(key, value).expect("TODO: panic message");
    config.refresh();
}
#[derive(Clone)]
pub struct Value<T> {
    pub v: T,
}
impl<T> Value<T> {
    pub fn new(v: T) -> Self {
        Value { v }
    }
    pub fn set(&mut self, v: T) {
        self.v = v;
    }
}
pub fn create_dir_if_all(path: &String) {
    if !Path::new(&path).exists() {
        fs::create_dir_all(path).expect(format!("Cannot create dir {}", path).as_str())
    }
}
pub fn get_user_name() -> String {
    whoami::username()
}