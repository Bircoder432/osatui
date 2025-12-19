pub mod main;
pub mod theme;

use ratatui::crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};

use crate::config::main::KeyMap;

use self::{main::MainConfig, theme::ThemeConfig};

#[derive(Clone, Deserialize)]
pub struct Config {
    main: MainConfig,
    themes: ThemeConfig,
}

impl Config {
    pub async fn load() -> anyhow::Result<Self> {
        let main = MainConfig::load().await?;
        let themes = ThemeConfig::load().await?;
        Ok(Self { main, themes })
    }
    pub fn default() -> Self {
        let main = MainConfig::default();
        let themes = ThemeConfig::default();
        Self { main, themes }
    }

    pub async fn save(&self) -> anyhow::Result<()> {
        self.main.save().await
    }

    pub fn api_url(&self) -> &str {
        &self.main.api.url
    }
    pub fn college_id(&self) -> u32 {
        self.main.api.college_id
    }
    pub fn college_name(&self) -> Option<&str> {
        self.main.api.college_name.as_deref()
    }
    pub fn campus_id(&self) -> u32 {
        self.main.api.campus_id
    }
    pub fn campus_name(&self) -> Option<&str> {
        self.main.api.campus_name.as_deref()
    }
    pub fn group_id(&self) -> u32 {
        self.main.api.group_id
    }
    pub fn group_name(&self) -> Option<&str> {
        self.main.api.group_name.as_deref()
    }
    pub fn cache_enabled(&self) -> bool {
        self.main.app.cache_enabled
    }
    pub fn cache_ttl(&self) -> u64 {
        self.main.app.cache_ttl
    }
    pub fn current_theme(&self) -> &str {
        &self.main.app.current_theme
    }
    pub fn theme(&self) -> &theme::Theme {
        self.themes.get(self.current_theme())
    }

    pub fn set_api_url(&mut self, url: String) {
        self.main.api.url = url.trim_end_matches('/').to_string();
    }

    pub fn set_college(&mut self, id: u32, name: Option<String>) {
        self.main.api.college_id = id;
        self.main.api.college_name = name;
    }

    pub fn set_campus(&mut self, id: u32, name: Option<String>) {
        self.main.api.campus_id = id;
        self.main.api.campus_name = name;
    }

    pub fn set_group(&mut self, id: u32, name: Option<String>) {
        self.main.api.group_id = id;
        self.main.api.group_name = name;
    }
    pub fn keymap(&self) -> KeyMap {
        self.main.keymap
    }
}
