pub mod keymap;
pub mod theme;

use crate::config::{
    keymap::KeyMap,
    theme::{Theme, ThemeManager},
};
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(flatten)]
    inner: ConfigData,
    #[serde(skip)]
    theme: Theme,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ConfigData {
    api: ApiConfig,
    app: AppConfig,
    keymap: KeyMap,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ApiConfig {
    url: String,
    college_id: u32,
    college_name: Option<String>,
    campus_id: u32,
    campus_name: Option<String>,
    group_id: u32,
    group_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AppConfig {
    refresh_interval: u64,
    cache_enabled: bool,
    cache_ttl: u64,
    current_theme: String,
}

impl Config {
    pub async fn load() -> anyhow::Result<Self> {
        let path = Self::config_path();

        let data: ConfigData = if path.exists() {
            let content = tokio::fs::read_to_string(&path)
                .await
                .with_context(|| format!("Failed to read config from {:?}", path))?;
            toml::from_str(&content).with_context(|| "Failed to parse config file")?
        } else {
            log::info!("Config not found, creating default at {:?}", path);
            let default = ConfigData::default();
            let config = Self {
                inner: default,
                theme: Theme::default(),
            };
            config.save().await?;
            return Ok(config);
        };
        let theme_name = &data.app.current_theme;
        log::info!("Loading theme {}", &theme_name);
        let theme = match ThemeManager::load().await {
            Ok(tm) => {
                log::info!("Theme {} loaded succesful", &theme_name);
                tm.get(&theme_name).unwrap_or(&Theme::default()).clone()
            }
            Err(e) => {
                log::warn!("Theme {} loading failed, Error: {}", &theme_name, e);
                Theme::default()
            }
        };

        Ok(Self {
            inner: data,
            theme: theme.clone(),
        })
    }

    pub async fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path();

        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .with_context(|| format!("Failed to create config directory {:?}", parent))?;
        }

        let toml =
            toml::to_string_pretty(&self.inner).with_context(|| "Failed to serialize config")?;

        tokio::fs::write(&path, toml)
            .await
            .with_context(|| format!("Failed to write config to {:?}", path))?;

        Ok(())
    }

    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| ".".into())
            .join("osatui/config.toml")
    }

    // Getters
    pub fn api_url(&self) -> &str {
        &self.inner.api.url
    }

    pub fn college_id(&self) -> u32 {
        self.inner.api.college_id
    }

    pub fn college_name(&self) -> Option<&str> {
        self.inner.api.college_name.as_deref()
    }

    pub fn campus_id(&self) -> u32 {
        self.inner.api.campus_id
    }

    pub fn campus_name(&self) -> Option<&str> {
        self.inner.api.campus_name.as_deref()
    }

    pub fn group_id(&self) -> u32 {
        self.inner.api.group_id
    }

    pub fn group_name(&self) -> Option<&str> {
        self.inner.api.group_name.as_deref()
    }

    pub fn cache_enabled(&self) -> bool {
        self.inner.app.cache_enabled
    }

    pub fn cache_ttl(&self) -> u64 {
        self.inner.app.cache_ttl
    }

    pub fn keymap(&self) -> &KeyMap {
        &self.inner.keymap
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    // Setters
    pub fn set_api_url(&mut self, url: String) {
        self.inner.api.url = url.trim_end_matches('/').to_string();
    }

    pub fn set_college(&mut self, id: u32, name: Option<String>) {
        self.inner.api.college_id = id;
        self.inner.api.college_name = name;
    }

    pub fn set_campus(&mut self, id: u32, name: Option<String>) {
        self.inner.api.campus_id = id;
        self.inner.api.campus_name = name;
    }

    pub fn set_group(&mut self, id: u32, name: Option<String>) {
        self.inner.api.group_id = id;
        self.inner.api.group_name = name;
    }
}

impl Default for ConfigData {
    fn default() -> Self {
        Self {
            api: ApiConfig {
                url: "https://api.thisishyum.ru/schedule_api/tyumen".to_string(),
                college_id: 1,
                college_name: None,
                campus_id: 1,
                campus_name: None,
                group_id: 1,
                group_name: None,
            },
            app: AppConfig {
                refresh_interval: 300,
                cache_enabled: true,
                cache_ttl: 3600,
                current_theme: "dark".to_string(),
            },
            keymap: KeyMap::default(),
        }
    }
}
