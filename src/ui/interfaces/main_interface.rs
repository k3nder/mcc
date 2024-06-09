use crate::deserialize::profiles_json::Profile;
use crate::logic;
use crate::logic::profile_management::{get_profiles, profiles_indexed_list, to_list_item};
use crate::logic::{to_list_state_valued, versions};
use crate::ui::components;
use crate::ui::components::list_item::ListItemValued;
use crate::ui::components::state_full_list::StatefulList;
use crate::ui::components::{shortcut, Component, Handle};
use crate::ui::interfaces::{profile_interface, settings_interface};
use crate::utils::console_log::LogType;
use crate::utils::{create_dir_if_all, replace_map, settings, settings_set, Value};
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use mclr::deserialize::json_version::JsonVersion;
use mclr::mc::mc::get_compatible_java;
use mclr::mc::utils::command_builder::{
    Command, CommandAssetsConfig, CommandRamConfig, CommandResourcesConfig, CommandUserConfig,
};
use mclr::utils::manifest::manifest;
use mclr::utils::sync_utils::sync;
use mclr::utils::HandleEvent;
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::format;
use std::fs::File;
use std::io::Read;
use std::ops::{AddAssign, Index};
use std::path::Path;
use std::process::exit;
use std::string::ToString;
use std::time::{Duration, Instant};
use std::{fs, io, thread};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, BorderType, Borders, Gauge, List, ListItem, Paragraph};
use tui::Terminal;

//static CONSOLE_HISTORY_CHANNEL: Lazy<(SyncSender<String>, Receiver<String>)> = Lazy::new(|| mpsc::sync_channel(0));
#[derive(Deserialize, Serialize, Debug)]
pub struct ConsoleCache {
    pub history: Vec<String>,
    pub percentage: u16,
}

// components
static mut CONSOLE_COMPONENT: Lazy<Component> = Lazy::new(|| {
    Component::new(vec![
        shortcut("[q]", " Quit"),
        shortcut("[←→↑↓]", " Nav in console"),
        shortcut("[TAB]", " Nav"),
        shortcut("[a]", " Anchor"),
        shortcut("[c]", " Clean"),
    ])
});

static mut USER_COMPONENT: Lazy<Component> =
    Lazy::new(|| Component::new(vec![shortcut("[q]", " Quit"), shortcut("[←→↑↓]", " Nav")]));
static mut PROFILES_COMPONENT: Lazy<Component> = Lazy::new(|| {
    Component::new(vec![
        shortcut("[q]", " Quit"),
        shortcut("[←→↑↓]", " Nav"),
        shortcut("[⏎]", " Execute"),
    ])
});
static mut USER_NAME_CONF_COMPONENT: Lazy<Component> = Lazy::new(|| {
    Component::new(vec![
        shortcut("[abc...]", " Write"),
        shortcut("[←→↑↓]", " Nav"),
    ])
});
// statics
static mut CONSOLE_ANCHOR: bool = true;
static mut USER_NAME_CONF: Lazy<String> = Lazy::new(|| settings("user.name"));
static mut PROFILES_LIST: Lazy<StatefulList<ListItem>> =
    Lazy::new(|| StatefulList::with_items(versions()));
static mut CONSOLE_HISTORY: Vec<Span> = vec![];
static mut CONSOLE_SCROLL_Y: u16 = 0;
static mut CONSOLE_SCROLL_X: u16 = 0;
static mut PROGRESS_BAR: u16 = 0;
static mut BAR_STATE: &str = "none/none";
static mut LAST_EVENT_TIME: Lazy<Value<Instant>> = Lazy::new(|| Value::new(Instant::now()));

pub unsafe fn interface() {
    init_components();
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let debounce_duration = Duration::from_millis(110); // Ajusta según sea necesario
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().expect("main clean");
    loop {
        terminal
            .draw(|f| {
                let size = f.size();
                /*
                the default chunks is 4 parts,
                * the interface selector
                * the progress bar
                * the main panels
                * the shortcuts list
                */
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Percentage(7),
                            Constraint::Percentage(6),
                            Constraint::Percentage(82),
                            Constraint::Percentage(5),
                        ]
                        .as_ref(),
                    )
                    .split(size);
                /*
                the profiles chunks is in 2 parts, the left part and the right part
                * Left is a list of profiles to select
                * Right is a console and the user settings
                */
                let profiles_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                    .split(chunks[2]);
                /*
                separated in 2 parts down and top.
                * Top part is the user settings
                * Down is the console
                */
                let user_settings_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(profiles_chunks[1]);
                // this is the progress bar chunks
                let progress_bar_chunks = Layout::default()
                    .margin(0)
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
                    .split(chunks[1]);
                let gauge = Gauge::default()
                    .block(Block::default().title("").borders(Borders::NONE))
                    .gauge_style(Style::default().fg(Color::Yellow))
                    .percent(PROGRESS_BAR);
                f.render_widget(gauge, progress_bar_chunks[0]);
                let state_progress_bar = Paragraph::new(BAR_STATE)
                    .block(Block::default().title("").borders(Borders::NONE));
                f.render_widget(state_progress_bar, progress_bar_chunks[1]);
                // this is a profiles list to select
                let profiles_list = PROFILES_LIST.clone();
                let profiles = List::new(profiles_list.items)
                    .block(
                        Block::default()
                            .title("profiles")
                            .borders(Borders::ALL)
                            .border_type(BorderType::Thick)
                            .border_style(PROFILES_COMPONENT.border()),
                    )
                    .highlight_style(
                        Style::default()
                            .bg(Color::LightYellow)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">> ");
                f.render_stateful_widget(profiles, profiles_chunks[0], &mut PROFILES_LIST.state);
                // this is a user settings
                let user_table = Block::default()
                    .title("User")
                    .borders(Borders::ALL)
                    .border_style(USER_COMPONENT.border());
                // these are a user settings inner chunks
                let user_inner_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(30),
                            Constraint::Percentage(20),
                            Constraint::Percentage(20),
                            Constraint::Percentage(20),
                            Constraint::Percentage(20),
                        ]
                        .as_ref(),
                    )
                    .split(user_settings_chunks[0]);
                // this is a one of the user settings
                let user_name_conf = Paragraph::new(Text::raw(&*USER_NAME_CONF)).block(
                    Block::default()
                        .title("username")
                        .borders(Borders::ALL)
                        .border_style(USER_NAME_CONF_COMPONENT.border()),
                );
                f.render_widget(user_name_conf, user_inner_chunks[0]);
                f.render_widget(user_table, user_settings_chunks[0]);

                // here is the console logic and the widget
                let history_text: Vec<Spans> = CONSOLE_HISTORY
                    .clone()
                    .into_iter()
                    .map(|entry| Spans::from(entry))
                    .collect();
                console_anchor_logic();
                let console_block = Paragraph::new(history_text)
                    .scroll((CONSOLE_SCROLL_Y, CONSOLE_SCROLL_X))
                    .block(
                        Block::default()
                            .title("Console")
                            .borders(Borders::ALL)
                            .border_style(CONSOLE_COMPONENT.border()),
                    )
                    .style(Style::default().fg(Color::White));
                f.render_widget(console_block, user_settings_chunks[1]);

                // here is the shortcuts
                let shortcuts = components::shortcuts_controller(vec![
                    CONSOLE_COMPONENT.clone(),
                    USER_COMPONENT.clone(),
                    PROFILES_COMPONENT.clone(),
                    USER_NAME_CONF_COMPONENT.clone(),
                ]);
                let flattened_shortcuts: Vec<Span> =
                    shortcuts.into_iter().flat_map(|v| v.into_iter()).collect();
                let help = Paragraph::new(Spans::from(flattened_shortcuts))
                    .block(Block::default().title("").borders(Borders::NONE))
                    .wrap(tui::widgets::Wrap { trim: false });
                f.render_widget(help, chunks[3]);
            })
            .unwrap();
        // this event listen the key event with crossterm
        key_event_listen(debounce_duration);
    }
}

unsafe fn key_event_listen(debounce_duration: Duration) {
    if event::poll(Duration::from_millis(10)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            let now = Instant::now();
            if now.duration_since(LAST_EVENT_TIME.clone().v) > debounce_duration {
                components::key_controller(
                    vec![
                        CONSOLE_COMPONENT.clone(),
                        USER_COMPONENT.clone(),
                        PROFILES_COMPONENT.clone(),
                        USER_NAME_CONF_COMPONENT.clone(),
                    ],
                    key_event,
                );
                match key_event.code {
                    KeyCode::F(2) => {
                        settings_interface::interface();
                    }
                    _ => {}
                }
                LAST_EVENT_TIME.set(now);
            }
        }
    }
}

unsafe fn console_anchor_logic() {
    if CONSOLE_ANCHOR {
        CONSOLE_SCROLL_Y = if CONSOLE_HISTORY.len() < 17 {
            0
        } else {
            CONSOLE_HISTORY.len() as u16 - 17
        };
    }
}

unsafe fn init_components() {
    init_profiles_component();
    init_user_component();
    init_name_conf_component();
    init_console_component();
}

unsafe fn init_console_component() {
    CONSOLE_COMPONENT.set_key_handle(Handle::new(move |e| {
        match e.code {
            KeyCode::Tab => {
                PROFILES_COMPONENT.set_active(true);
                CONSOLE_COMPONENT.set_active(false);
            }
            KeyCode::Left => {
                if CONSOLE_SCROLL_X != 0 {
                    CONSOLE_SCROLL_X -= 1;
                }
            }
            KeyCode::Right => {
                CONSOLE_SCROLL_X += 1;
            }
            KeyCode::Down => {
                CONSOLE_ANCHOR = false;
                CONSOLE_SCROLL_Y += 1;
            }
            KeyCode::Char('a') => {
                CONSOLE_ANCHOR = true;
            }
            KeyCode::Up => {
                CONSOLE_ANCHOR = false;
                if CONSOLE_SCROLL_Y == 0 {
                    return;
                }
                CONSOLE_SCROLL_Y -= 1;
            }
            KeyCode::Char('c') => {
                CONSOLE_HISTORY = vec![];
            }
            KeyCode::Char('q') => exit(0),
            _ => {
                //CONSOLE_HISTORY.clone().lock().unwrap().push(format!("{:?}", e.code));
            }
        }
    }));
}

unsafe fn init_name_conf_component() {
    USER_NAME_CONF_COMPONENT.set_key_handle(Handle::new(|e| match e.code {
        KeyCode::Char(c) => {
            USER_NAME_CONF.push(c);
            settings_set("user.name", USER_NAME_CONF.as_str());
        }
        KeyCode::Backspace => {
            USER_NAME_CONF.pop();
            settings_set("user.name", USER_NAME_CONF.as_str());
        }
        KeyCode::Down => {
            USER_COMPONENT.set_active(true);
            USER_NAME_CONF_COMPONENT.set_active(false);
        }
        _ => {}
    }));
}

unsafe fn init_user_component() {
    USER_COMPONENT.set_key_handle(Handle::new(move |e| match e.code {
        KeyCode::Char('q') => exit(0),
        KeyCode::Right => {
            CONSOLE_COMPONENT.set_active(true);
            USER_COMPONENT.set_active(false);
        }
        KeyCode::Left => {
            PROFILES_COMPONENT.set_active(true);
            USER_COMPONENT.set_active(false);
        }
        KeyCode::Down => {
            USER_NAME_CONF_COMPONENT.set_active(true);
            USER_COMPONENT.set_active(false);
        }
        _ => {}
    }));
}

unsafe fn init_profiles_component() {
    PROFILES_COMPONENT.set_active(true);
    PROFILES_COMPONENT.set_key_handle(Handle::new(move |e| match e.code {
        KeyCode::Char('q') => exit(0),
        KeyCode::Char('c') => {
            profile_interface::interface();
        }
        KeyCode::Down => PROFILES_LIST.next(),
        KeyCode::Up => PROFILES_LIST.previous(),

        KeyCode::Enter => {
            thread::spawn(move || {
                let index = PROFILES_LIST.state.selected().unwrap();
                let binding = sync().block_on(manifest());
                let profile_list_item = &binding.versions.get(index).unwrap().id;
                mc_init(&profile_list_item);
            });
        }
        KeyCode::Right => {
            USER_COMPONENT.set_active(true);
            PROFILES_COMPONENT.set_active(false);
        }
        _ => {}
    }));
}

unsafe fn mc_init(last_version_id: &str) {
    let versions_path = &settings("versions.path");
    let java_homes = &settings("java.homes");
    let manifest_url = settings("manifest.url");
    let assets_url = settings("assets.url");
    let _versions_bin = settings("versions.bin");
    let assets_dir = &settings("assets.path");
    let libs_dir = settings("libs.dir");
    let versions_json_file = settings("versions.json.file");
    let game_dir = &settings("game.dir");
    let versions_jar_files = settings("versions.jar.file");
    // create settings dirs
    create_dir_if_all(versions_path);
    create_dir_if_all(java_homes);
    create_dir_if_all(assets_dir);
    create_dir_if_all(game_dir);
    console_management::clean();
    console_management::log(
        format!("downloading.. {}.", &last_version_id.clone()),
        LogType::CHAT,
    );
    PROGRESS_BAR = 0;
    BAR_STATE = " :Checking manifest";
    //CONSOLE_HISTORY.push(Span::styled(format!("Manifest: {}", manifest_url), Style::default().fg(Color::Gray)));
    //CONSOLE_HISTORY.push(Span::styled(format!("Assets dir: {}", assets_dir), Style::default().fg(Color::Gray)));
    let _manifest_sync =
        sync().block_on(mclr::utils::manifest::manifest_url(manifest_url.as_str()));
    let _version_m = _manifest_sync.get(last_version_id.clone()).unwrap();
    let version_id = &_version_m.id;
    PROGRESS_BAR += 25;
    BAR_STATE = " :client.json 1/5";
    //let version_id = "24w14potato";
    // build the map than contains the values
    let mut map_temp: HashMap<String, String> = HashMap::new();
    map_temp.insert("var.version.id".to_string(), version_id.clone().to_string());

    create_dir_if_all(&format!("{}/{}", versions_path, version_id));
    create_dir_if_all(&replace_map(&libs_dir.clone(), map_temp.clone()));

    let version = _version_m
        .save_and_load(replace_map(versions_json_file.as_str(), map_temp.clone()).as_str());
    PROGRESS_BAR += 25;
    BAR_STATE = " :java 2/5";
    //CONSOLE_HISTORY.push(format!("versions_bin: {}; libs_dir: {}; version_json_file: {};", versions_bin, libs_dir, version_json_file));
    //CONSOLE_HISTORY.push(format!("Json: {}", replace_map(versions_json_file.as_str(), map_temp.clone())));

    //let version: JsonVersion = serde_json::from_str(&*read(replace_map(versions_json_file.clone().as_str(), map_temp.clone()).as_str())).unwrap();
    let java_home = get_compatible_java(java_homes, &version.javaVersion);
    PROGRESS_BAR += 25;
    BAR_STATE = " :jar 3/5";

    // paths and parameters

    mclr::mc::mc::download(
        replace_map(versions_jar_files.as_str(), map_temp.clone()).as_str(),
        &version,
    );
    PROGRESS_BAR += 25;
    BAR_STATE = " :assets 4/5";
    if !mclr::mc::utils::assets_utils::verify(assets_dir, &version, HandleEvent::new(move |e| {})) {
        mclr::mc::utils::assets_utils::download_all_url(
            assets_dir,
            &version,
            HandleEvent::new(move |e| {
                PROGRESS_BAR = e.percent() as u16;
            }),
            HandleEvent::new(|e| {
                CONSOLE_HISTORY.push(Span::styled(
                    format!("Downloading: {}", e),
                    Style::default().fg(Color::LightGreen),
                ));
            }),
            assets_url.as_str(),
        );
    }
    BAR_STATE = " :libraries 5/5";
    mclr::mc::utils::libs_utils::get_libs(
        replace_map(libs_dir.as_str(), map_temp.clone()).as_str(),
        replace_map(_versions_bin.clone().as_str(), map_temp.clone()).as_str(),
        &version.libraries,
        HandleEvent::new(move |e| {
            PROGRESS_BAR = e.percent() as u16;
        }),
    )
    .expect("TODO: panic message");
    CONSOLE_HISTORY = vec![];
    BAR_STATE = " :Running";
    PROGRESS_BAR = 0;

    // user data
    let user_name = settings("user.name");

    Command {
        resources: CommandResourcesConfig {
            libraries: replace_map(libs_dir.as_str(), map_temp.clone()),
            jar_file: replace_map(versions_jar_files.as_str(), map_temp.clone())
                .as_str()
                .to_string(),
            bin: replace_map(_versions_bin.clone().as_str(), map_temp.clone()).to_string(),
        },
        java_home,
        game_dir: game_dir.to_string(),
        assets: CommandAssetsConfig {
            assets_dir: assets_dir.to_string(),
            assets_index: version.assets.to_string(),
        },
        user: CommandUserConfig {
            user_type: USER_NAME_CONF.clone(),
            client_id: "0".to_string(),
            uuid: "d0db8a3d-c392-4ae7-96e5-9365de33ab52".to_string(),
            xuid: "0".to_string(),
            access_token: "0".to_string(),
            user_name,
        },
        version: version.command_conf(),
        ram: CommandRamConfig { xmx: 4, xms: 2 },
        event: |s| {
            CONSOLE_HISTORY.push(Span::styled(
                s.clone(),
                LogType::get_of(s.clone()).get_style(),
            ));
        },
    }
    .run();
}

fn read(file: &str) -> String {
    let mut file = File::open(file).unwrap();
    let mut content = "".to_string();
    file.read_to_string(&mut content)
        .expect("TODO: panic message");
    content
}

mod console_management {
    use crate::ui::interfaces::main_interface::CONSOLE_HISTORY;
    use crate::utils::console_log;
    use crate::utils::console_log::LogType;
    use tui::text::Span;

    pub unsafe fn log(str: String, log: LogType) {
        CONSOLE_HISTORY.push(Span::styled(str.clone(), log.get_style()));
    }

    pub unsafe fn clean() {
        CONSOLE_HISTORY = vec![];
    }
}
