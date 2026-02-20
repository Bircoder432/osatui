use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Theme {
    background: String,
    text: String,
    header_bg: String,
    header_fg: String,
    table_header: String,
    border: String,
    highlight: String,
    error: String,
}

impl Theme {
    pub fn background_color(&self) -> Color {
        self.parse_color(&self.background)
    }

    pub fn text_color(&self) -> Color {
        self.parse_color(&self.text)
    }

    pub fn header_bg_color(&self) -> Color {
        self.parse_color(&self.header_bg)
    }

    pub fn header_fg_color(&self) -> Color {
        self.parse_color(&self.header_fg)
    }

    pub fn table_header_color(&self) -> Color {
        self.parse_color(&self.table_header)
    }

    pub fn border_color(&self) -> Color {
        self.parse_color(&self.border)
    }

    pub fn highlight_color(&self) -> Color {
        self.parse_color(&self.highlight)
    }

    pub fn error_color(&self) -> Color {
        self.parse_color(&self.error)
    }

    fn parse_color(&self, hex: &str) -> Color {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Color::White;
        }

        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);

        Color::Rgb(r, g, b)
    }
}

pub struct ThemeManager {
    themes: HashMap<String, Theme>,
}

impl ThemeManager {
    pub async fn load() -> anyhow::Result<Self> {
        let path = Self::theme_path();

        let themes: HashMap<String, Theme> = if path.exists() {
            let content = tokio::fs::read_to_string(&path).await?;
            toml::from_str(&content)?
        } else {
            HashMap::new()
        };

        Ok(Self { themes })
    }
    pub fn get(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }

    fn theme_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| ".".into())
            .join("osatui/theme.toml")
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: "#1e1e1e".to_string(),
            text: "#dcdcdc".to_string(),
            header_bg: "#0064c8".to_string(),
            header_fg: "#ffffff".to_string(),
            table_header: "#ffff00".to_string(),
            border: "#646464".to_string(),
            highlight: "#00c800".to_string(),
            error: "#ff0000".to_string(),
        }
    }
}
