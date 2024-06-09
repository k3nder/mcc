pub mod cache_temp;
mod interfaces;
pub mod profile_management;

use crate::deserialize::profiles_json::{Profile, Profiles};
use crate::ui::components::list_item::ListItemValued;
use crate::utils::settings;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use mclr::utils::manifest::{manifest, manifest_url};
use mclr::utils::sync_utils::sync;
use std::collections::HashMap;
use std::fmt::format;
use std::process::exit;
use tui::widgets::{ListItem, StatefulWidget};

pub fn versions() -> Vec<ListItem<'static>> {
    let binding = settings("manifest.url");
    let manifest_async = manifest_url(binding.as_str());
    let manifest = sync().block_on(manifest_async);
    let mut result: Vec<ListItem> = vec![];
    for version in manifest.versions {
        result.push(ListItem::new(format!("{}", version.id)));
    }
    result
}

pub fn to_list_state_valued(list: Vec<Profile>) -> Vec<ListItemValued<Profile>> {
    let mut result = vec![];
    for profile in list {
        result.push(ListItemValued::new(
            ListItem::new(format!("{} : {}", profile.name, profile.typ)),
            profile,
        ));
    }
    result
}
