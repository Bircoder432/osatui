pub mod main;
pub mod theme;

use self::{main::MainConfig, theme::ThemeConfig};

#[derive(Clone)]
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

    pub fn api_url(&self) -> &str {
        &self.main.api.url
    }
    pub fn college_id(&self) -> u32 {
        self.main.api.college_id
    }
    pub fn group_id(&self) -> u32 {
        self.main.api.group_id
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
}
