use crate::utils::date::AppDate;
use osars::models::Schedule;
use std::path::PathBuf;

pub struct CacheManager {
    dir: PathBuf,
    ttl: u64,
}

impl CacheManager {
    pub async fn new(ttl: u64) -> anyhow::Result<Self> {
        let dir = dirs::cache_dir()
            .unwrap_or_else(|| ".".into())
            .join("osatui");
        tokio::fs::create_dir_all(&dir).await?;
        Ok(Self { dir, ttl })
    }

    pub async fn get(&self, date: &AppDate) -> anyhow::Result<Option<Vec<Schedule>>> {
        let path = self.dir.join(format!("{}.json", date.iso()));
        if !path.exists() {
            return Ok(None);
        }
        let content = tokio::fs::read_to_string(&path).await?;
        let data: Vec<Schedule> = serde_json::from_str(&content)?;
        Ok(Some(data))
    }

    pub async fn set(&self, date: &AppDate, data: &[Schedule]) -> anyhow::Result<()> {
        let path = self.dir.join(format!("{}.json", date.iso()));
        let content = serde_json::to_string_pretty(data)?;
        tokio::fs::write(&path, content).await?;
        Ok(())
    }

    pub async fn clear(&self) -> anyhow::Result<()> {
        if self.dir.exists() {
            for entry in std::fs::read_dir(&self.dir)? {
                let entry = entry?;
                if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                    tokio::fs::remove_file(entry.path()).await?;
                }
            }
        }
        Ok(())
    }
}
