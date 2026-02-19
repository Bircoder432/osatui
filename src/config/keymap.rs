use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyMap {
    prev_day: KeyMapEntry,
    cur_day: KeyMapEntry,
    next_day: KeyMapEntry,
    selector: KeyMapEntry,
    settings: KeyMapEntry,
    exit: KeyMapEntry,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum KeyMapEntry {
    Simple(String),
    WithModifier { key: String, ctrl: bool, alt: bool },
}

impl KeyMap {
    pub fn prev_day(&self) -> KeyCode {
        self.prev_day.parse()
    }

    pub fn cur_day(&self) -> KeyCode {
        self.cur_day.parse()
    }

    pub fn next_day(&self) -> KeyCode {
        self.next_day.parse()
    }

    pub fn selector(&self) -> KeyCode {
        self.selector.parse()
    }

    pub fn settings(&self) -> KeyCode {
        self.settings.parse()
    }

    pub fn exit(&self) -> KeyCode {
        self.exit.parse()
    }
}

impl KeyMapEntry {
    fn parse(&self) -> KeyCode {
        let (key_str, _ctrl, _alt) = match self {
            KeyMapEntry::Simple(s) => (s.as_str(), false, false),
            KeyMapEntry::WithModifier { key, ctrl, alt } => (key.as_str(), *ctrl, *alt),
        };

        match key_str {
            "Left" | "left" => KeyCode::Left,
            "Right" | "right" => KeyCode::Right,
            "Up" | "up" => KeyCode::Up,
            "Down" | "down" => KeyCode::Down,
            "Enter" | "return" => KeyCode::Enter,
            "Esc" | "escape" => KeyCode::Esc,
            "Tab" => KeyCode::Tab,
            "BackTab" => KeyCode::BackTab,
            "Backspace" => KeyCode::Backspace,
            "F1" => KeyCode::F(1),
            "F2" => KeyCode::F(2),
            "F3" => KeyCode::F(3),
            s if s.len() == 1 => KeyCode::Char(s.chars().next().unwrap()),
            _ => KeyCode::Null,
        }
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        Self {
            prev_day: KeyMapEntry::Simple("Left".to_string()),
            cur_day: KeyMapEntry::Simple("Up".to_string()),
            next_day: KeyMapEntry::Simple("Right".to_string()),
            selector: KeyMapEntry::Simple("o".to_string()),
            settings: KeyMapEntry::Simple("s".to_string()),
            exit: KeyMapEntry::Simple("q".to_string()),
        }
    }
}
