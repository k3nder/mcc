use crate::deserialize::profiles_json::Profile;
use crate::ui::components;
use crate::ui::components::{shortcut, Component, Handle};
use crate::ui::interfaces::main_interface;
use crate::utils::Value;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use once_cell::sync::Lazy;
use std::io;
use std::time::{Duration, Instant};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;

static mut LAST_EVENT_TIME: Lazy<Value<Instant>> = Lazy::new(|| Value::new(Instant::now()));
static mut PROFILE: Lazy<Profile> = Lazy::new(|| Profile::default());
static mut PROFILE_NAME_C: Lazy<Component> =
    Lazy::new(|| Component::new(vec![shortcut("[abc..]", " Write")]));
static mut PROFILE_VERSION_C: Lazy<Component> =
    Lazy::new(|| Component::new(vec![shortcut("[abc..]", " Write")]));
static mut PROFILE_GAMEDIR_C: Lazy<Component> =
    Lazy::new(|| Component::new(vec![shortcut("[abc..]", " Write")]));

pub enum Type {
    EDIT,
    NEW,
}

pub unsafe fn interface() {
    PROFILE = Lazy::new(|| Profile::default());
    init_components();
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let debounce_duration = Duration::from_millis(20); // Ajusta según sea necesario
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().expect("profiles_clean clean");
    PROFILE_NAME_C.set_active(true);
    loop {
        terminal
            .draw(|f| {
                let size = f.size();
                let chunks = Layout::default()
                    .margin(0)
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(100)])
                    .split(size);
                let principal_block = Block::default().borders(Borders::ALL).title("dd");
                f.render_widget(principal_block, chunks[0]);
                let inner_chunks = Layout::default()
                    .margin(1)
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(5), Constraint::Percentage(5)])
                    .split(chunks[0]);
                let name_paragraph = Paragraph::new(PROFILE.clone().name).block(
                    Block::default()
                        .title("name")
                        .borders(Borders::ALL)
                        .border_style(PROFILE_NAME_C.border()),
                );
                let version_paragraph = Paragraph::new(PROFILE.clone().lastVersionId).block(
                    Block::default()
                        .title("version")
                        .borders(Borders::ALL)
                        .border_style(PROFILE_VERSION_C.border()),
                );
                f.render_widget(name_paragraph, inner_chunks[0]);
                f.render_widget(version_paragraph, inner_chunks[1]);
            })
            .unwrap();
        key_event_listen(debounce_duration);
    }
}

pub unsafe fn init_components() {
    profile_name_component_init();
    profile_version_component_init();
}

unsafe fn profile_version_component_init() {
    PROFILE_VERSION_C.set_key_handle(Handle::new(|f| match f.code {
        KeyCode::Char(c) => {
            PROFILE.lastVersionId.push(c);
        }
        KeyCode::Backspace => {
            PROFILE.lastVersionId.pop();
        }
        _ => {}
    }))
}

unsafe fn profile_name_component_init() {
    PROFILE_NAME_C.set_key_handle(Handle::new(|c| match c.code {
        KeyCode::Char(c) => PROFILE.name.push(c),
        KeyCode::Backspace => {
            PROFILE.name.pop();
        }
        KeyCode::Down => {
            PROFILE_VERSION_C.set_active(true);
            PROFILE_NAME_C.set_active(false);
        }
        _ => {}
    }));
}

unsafe fn key_event_listen(debounce_duration: Duration) {
    if event::poll(Duration::from_millis(10)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            let now = Instant::now();
            if now.duration_since(LAST_EVENT_TIME.clone().v) > debounce_duration {
                components::key_controller(
                    vec![PROFILE_NAME_C.clone(), PROFILE_VERSION_C.clone()],
                    key_event,
                );
                match key_event.code {
                    KeyCode::Tab => main_interface::interface(),
                    _ => {}
                }
                LAST_EVENT_TIME.set(now);
            }
        }
    }
}
