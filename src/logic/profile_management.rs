use crate::deserialize::profiles_json::{Profile, Profiles};
use crate::ui::components::list_item::ListItemValued;
use crate::utils::settings;
use std::collections::HashMap;
use tui::widgets::ListItem;

pub fn profiles_list() -> Vec<ListItem<'static>> {
    let parsed = get_profiles();
    let mut result: Vec<ListItem> = vec![];
    for profile in parsed.profiles {
        result.push(ListItem::new(format!(
            "{} : {} : {}",
            profile.0, profile.1.lastVersionId, profile.1.typ
        )));
    }
    result
}
pub fn to_list_item(list: Vec<ListItemValued<Profile>>) -> Vec<ListItem<'static>> {
    let mut result: Vec<ListItem> = vec![];
    for profile_l in list {
        let profile = profile_l.get_value();
        result.push(ListItem::new(format!(
            "{} : {} : {}",
            profile.name, profile.lastVersionId, profile.typ
        )));
    }
    result
}
pub fn get_profiles() -> Profiles {
    let file_path = settings("profiles.file");
    let parsed = Profiles::serialize(file_path.as_str());
    return parsed;
}
pub fn profiles_indexed_list() -> Vec<Profile> {
    let profiles = get_profiles();
    let mut result: Vec<Profile> = vec![];
    for (key, profile) in profiles.profiles {
        result.push(profile);
    }
    result
}
pub fn add_new(profile: Profile) {
    let mut profiles = get_profiles();
    let name = &profile.clone().name;
    profiles.add(profile, name);
    profiles.save(settings("profiles.file").as_str());
}
