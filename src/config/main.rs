use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Api {
    pub url: String,
    pub college_id: u32,
    pub campus_id: u32,
    pub group_id: u32,
}
#[derive(Debug, Deserialize, Clone)]
pub struct App {
    pub refresh_interval: u64,
    pub cache_enabled: bool,
    pub cache_ttl: u64,
    pub current_theme: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MainConfig {
    #[serde(rename = "api")]
    pub api: Api,
    #[serde(rename = "app")]
    pub app: App,
}

impl MainConfig {
    pub async fn load() -> anyhow::Result<Self> {
        let path = dirs::config_dir()
            .unwrap_or_else(|| ".".into())
            .join("osatui/config.toml");

        let s = if path.exists() {
            tokio::fs::read_to_string(&path).await?
        } else {
            tokio::fs::read_to_string("config.toml").await?
        };

        let mut cfg: Self = toml::from_str(&s)?;
        cfg.api.url = cfg.api.url.trim_end_matches('/').to_string();
        Ok(cfg)
    }
}
