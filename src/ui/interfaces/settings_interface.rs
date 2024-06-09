use crate::ui::components;
use crate::ui::components::{shortcut, Component, Handle};
use crate::ui::interfaces::main_interface;
use crate::utils::settings;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use once_cell::sync::Lazy;
use std::io;
use std::time::{Duration, Instant};
use toml::value::Offset::Z;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders};
use tui::Terminal;
// components
static mut DANGER_SETTINGS_COMPONENT: Lazy<Component> =
    Lazy::new(|| Component::new(vec![shortcut("[]", " Nav")]));

pub unsafe fn interface() {
    init_components();
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let last_event_time = Instant::now();
    let debounce_duration = Duration::from_millis(20); // Ajusta según sea necesario
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().expect("settings clean");
    loop {
        terminal
            .draw(|f| {
                let size = f.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(10),
                            Constraint::Percentage(70),
                            Constraint::Percentage(10),
                            Constraint::Percentage(10),
                        ]
                        .as_ref(),
                    )
                    .split(size);
                let settings_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(chunks[1]);
                let danger_settings = Block::default()
                    .title("Danger settings")
                    .borders(Borders::ALL)
                    .border_style(DANGER_SETTINGS_COMPONENT.border());
                f.render_widget(danger_settings, settings_chunks[0]);

                let user_settings = Block::default()
                    .title("User settings")
                    .borders(Borders::ALL);
                f.render_widget(user_settings, settings_chunks[1])
            })
            .unwrap();
        key_event_listen(last_event_time, debounce_duration);
    }
}
pub unsafe fn init_components() {
    init_danger_settings_component();
}
pub unsafe fn init_danger_settings_component() {
    DANGER_SETTINGS_COMPONENT.set_key_handle(Handle::new(|e| match e.code {
        _ => {}
    }));
}
unsafe fn key_event_listen(mut last_event_time: Instant, debounce_duration: Duration) {
    if event::poll(Duration::from_millis(100)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            let now = Instant::now();
            if now.duration_since(last_event_time) > debounce_duration {
                components::key_controller(vec![DANGER_SETTINGS_COMPONENT.clone()], key_event);
                match key_event.code {
                    KeyCode::F(1) => main_interface::interface(),
                    _ => {}
                }
                last_event_time = now;
            }
        }
    }
}
