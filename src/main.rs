#[macro_use]
extern crate lazy_static;

use crate::deserialize::mcc_toml::MccToml;
use crate::deserialize::profiles_json::{Profile, Profiles};
use crate::logic::cache_temp::Cache;
use crate::logic::profile_management;
use crate::ui::interfaces::main_interface;
use crate::utils::settings;
use config::{Config, FileFormat};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{fs, io};

mod deserialize;
mod logic;
mod ui;
mod utils;

fn create_ini_file_if_not_exists(file_path: &str, content: &str) -> io::Result<()> {
    if !std::path::Path::new(file_path).exists() {
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
    }
    Ok(())
}

fn main() {
    let content = r#"
java.homes=[env.dir]/java
versions.path=[appdata.roaming]/.minecraft/versions
profiles.file=[appdata.roaming]/.minecraft/launcher_profiles.json
manifest.url=https://launchermeta.mojang.com/mc/game/version_manifest_v2.json
assets.url=https://resources.download.minecraft.net
versions.bin=[appdata.roaming]/.minecraft/versions/[var.version.id]/bin
assets.path=[appdata.roaming]/.minecraft/assets
libs.dir=[appdata.roaming]/.minecraft/versions/[var.version.id]/libraries
versions.json.file=[appdata.roaming]/.minecraft/versions/[var.version.id]/[var.version.id].json
versions.jar.file=[appdata.roaming]/.minecraft/versions/[var.version.id]/[var.version.id].jar
game.dir=[appdata.roaming]/.minecraft

[user]
name=user
uuid=112
xuid=23
xmx=4
xms=2
client.id=2
access.token=0
type=user
"#;

    let file_path = ".ini";

    if let Err(e) = create_ini_file_if_not_exists(file_path, content) {
        eprintln!("Error creating file: {}", e);
    } else {
        println!("File created successfully!");
    }

    unsafe {
        main_interface::interface();
    }
}
