use crossterm::event::KeyEvent;
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{StatefulWidget, Widget};
pub mod list_item;
pub mod state_full_list;

#[derive(Clone)]
pub struct Component<'a> {
    active: bool,
    shortcuts: Vec<Vec<Span<'a>>>,
    key_event: Handle<KeyEvent>,
}
impl<'a> Component<'a> {
    pub fn new(shortcuts: Vec<Vec<Span>>) -> Component {
        Component {
            shortcuts,
            key_event: Handle { event: |f| {} },
            active: false,
        }
    }
    pub fn set_key_handle(&mut self, a: Handle<KeyEvent>) {
        self.key_event = a;
    }
    pub fn border(&self) -> Style {
        let mut color;
        if self.active {
            color = Color::White
        } else {
            color = Color::Gray
        }
        Style::default().fg(color)
    }
    pub fn get_shortcuts(self) -> Vec<Vec<Span<'a>>> {
        self.shortcuts
    }
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
    pub fn active(self) -> bool {
        self.active
    }
    pub fn launch_key_event(&self, event: KeyEvent) {
        (self.key_event.event)(event);
    }
}
pub fn shortcut<'a>(key: &'a str, description: &'a str) -> Vec<Span<'a>> {
    vec![
        Span::styled(
            key,
            Style::default()
                .bg(Color::Rgb(19, 78, 191))
                .fg(Color::Rgb(251, 251, 238))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            description,
            Style::default()
                .bg(Color::Rgb(19, 78, 191))
                .fg(Color::Rgb(251, 251, 238)),
        ),
        Span::raw("  "),
    ]
}
pub fn key_controller(components: Vec<Component>, key_event: KeyEvent) {
    for c in components {
        if c.active {
            (c.key_event.event)(key_event);
        }
    }
}
pub fn shortcuts_controller(components: Vec<Component>) -> Vec<Vec<Span>> {
    let mut result = vec![];
    for c in components {
        if c.active {
            return c.shortcuts;
        }
    }
    result
}
#[derive(Clone)]
pub struct Handle<T> {
    event: fn(T),
}
impl<T> Handle<T> {
    pub fn new(event: fn(T)) -> Self {
        Handle { event }
    }
}
