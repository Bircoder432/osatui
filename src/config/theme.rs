use ratatui::style::Color;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Theme {
    pub background: String,
    pub text: String,
    pub header_bg: String,
    pub header_fg: String,
    pub table_header: String,
    pub border: String,
    pub highlight: String,
}

impl Theme {
    pub fn color(&self, field: &str) -> Color {
        let hex = match field {
            "background" => &self.background,
            "text" => &self.text,
            "header_bg" => &self.header_bg,
            "header_fg" => &self.header_fg,
            "table_header" => &self.table_header,
            "border" => &self.border,
            "highlight" => &self.highlight,
            _ => "#ffffff",
        };
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
        Color::Rgb(r, g, b)
    }
}

#[derive(Clone)]
pub struct ThemeConfig(HashMap<String, Theme>);

impl ThemeConfig {
    pub async fn load() -> anyhow::Result<Self> {
        let path = dirs::config_dir()
            .unwrap_or_else(|| ".".into())
            .join("osatui/theme.toml");

        let s = if path.exists() {
            tokio::fs::read_to_string(&path).await?
        } else {
            tokio::fs::read_to_string("theme.toml").await?
        };

        let mut themes: HashMap<String, Theme> = toml::from_str(&s)?;
        if !themes.contains_key("dark") {
            themes.insert(
                "dark".into(),
                Theme {
                    background: "#1e1e1e".into(),
                    text: "#dcdcdc".into(),
                    header_bg: "#0064c8".into(),
                    header_fg: "#ffffff".into(),
                    table_header: "#ffff00".into(),
                    border: "#646464".into(),
                    highlight: "#00c800".into(),
                },
            );
        }
        Ok(Self(themes))
    }

    pub fn get(&self, name: &str) -> &Theme {
        self.0.get(name).unwrap_or(&self.0["dark"])
    }
}
