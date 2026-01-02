use crossterm::event::KeyCode;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Api {
    pub url: String,
    pub college_id: u32,
    pub college_name: Option<String>,
    pub campus_id: u32,
    pub campus_name: Option<String>,
    pub group_id: u32,
    pub group_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct App {
    pub refresh_interval: u64,
    pub cache_enabled: bool,
    pub cache_ttl: u64,
    pub current_theme: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct KeyMap {
    pub prev_day: KeyCode,
    pub cur_day: KeyCode,
    pub next_day: KeyCode,
    pub selector: KeyCode,
    pub settings: KeyCode,
    pub exit: KeyCode,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MainConfig {
    #[serde(rename = "api")]
    pub api: Api,
    #[serde(rename = "app")]
    pub app: App,
    #[serde(rename = "keymap")]
    pub keymap: KeyMap,
}

impl MainConfig {
    pub async fn load() -> anyhow::Result<Self> {
        let path = Self::config_path();

        if path.exists() {
            info!("config founded");
            let s = tokio::fs::read_to_string(&path).await?;
            let mut cfg: Self = toml::from_str(&s)?;
            cfg.api.url = cfg.api.url.trim_end_matches('/').to_string();
            Ok(cfg)
        } else {
            warn!("config not founded use default");
            let default_config = Self::default();
            default_config.save().await?;
            Ok(default_config)
        }
    }

    pub async fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let toml = toml::to_string_pretty(self)?;
        fs::write(&path, toml).await?;
        Ok(())
    }

    fn config_path() -> std::path::PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| ".".into())
            .join("osatui/config.toml")
    }

    pub fn default() -> Self {
        Self {
            api: Api {
                url: "https://api.thisishyum.ru/schedule_api/tymen".to_string(),
                college_id: 1,
                college_name: None,
                campus_id: 1,
                campus_name: None,
                group_id: 1,
                group_name: None,
            },
            app: App {
                refresh_interval: 300,
                cache_enabled: true,
                cache_ttl: 3600,
                current_theme: "dark".to_string(),
            },
            keymap: KeyMap {
                prev_day: KeyCode::Left,
                cur_day: KeyCode::Up,
                next_day: KeyCode::Right,
                selector: KeyCode::Char('o'),
                settings: KeyCode::Char('s'),
                exit: KeyCode::Char('q'),
            },
        }
    }
}
