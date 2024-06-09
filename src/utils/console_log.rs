use tui::style::{Color, Modifier, Style};

pub enum LogType {
    INFO,
    WARN,
    ERROR,
    CHAT,
    DEAD,
    STOP,
}

impl LogType {
    pub fn get_of(s: String) -> LogType {
        return if s.contains("INFO]: [CHAT]") || s.contains("INFO]: [System] [CHAT]") {
            LogType::CHAT
        } else if s.contains("ERROR]: ") {
            LogType::ERROR
        } else if s.contains("WARN]: ") {
            LogType::WARN
        } else if s.contains("INFO]: Stopping!") {
            LogType::STOP
        } else if s.contains("INFO]: ") {
            LogType::INFO
        } else {
            LogType::INFO
        };
    }
    pub fn get_style(&self) -> Style {
        return match self {
            LogType::WARN => Style::default().fg(Color::Yellow),
            LogType::ERROR => Style::default().fg(Color::Red),
            LogType::CHAT => Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            LogType::STOP => Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::ITALIC),
            _ => Style::default().fg(Color::Gray),
        };
    }
}
