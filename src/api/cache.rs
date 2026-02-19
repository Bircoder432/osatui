use crate::utils::AppDate;
use osars::models::Schedule;
use std::path::PathBuf;
use std::time::SystemTime;

pub struct CacheManager {
    dir: PathBuf,
    ttl: u64,
    group_id: Option<u32>,
}

impl CacheManager {
    pub async fn new(ttl: u64) -> anyhow::Result<Self> {
        let dir = dirs::cache_dir()
            .unwrap_or_else(|| ".".into())
            .join("osatui");

        tokio::fs::create_dir_all(&dir).await?;

        Ok(Self {
            dir,
            ttl,
            group_id: None,
        })
    }

    pub fn update_ttl(&mut self, ttl: u64) {
        self.ttl = ttl;
    }

    pub fn set_group_id(&mut self, group_id: u32) {
        self.group_id = Some(group_id);
    }

    fn cache_file_name(&self, date: &AppDate) -> String {
        match self.group_id {
            Some(gid) => format!("{}-{}.json", gid, date.iso()),
            None => format!("{}.json", date.iso()),
        }
    }

    pub async fn get(&self, date: &AppDate) -> anyhow::Result<Option<Vec<Schedule>>> {
        let file_name = self.cache_file_name(date);
        let path = self.dir.join(&file_name);

        if !path.exists() {
            return Ok(None);
        }

        let metadata = tokio::fs::metadata(&path).await?;
        let modified = metadata.modified()?;
        let age = SystemTime::now().duration_since(modified)?.as_secs();

        if age > self.ttl {
            tokio::fs::remove_file(&path).await?;
            return Ok(None);
        }

        let content = tokio::fs::read_to_string(&path).await?;
        let data: Vec<Schedule> = serde_json::from_str(&content)?;

        Ok(Some(data))
    }

    pub async fn set(&self, date: &AppDate, data: &[Schedule]) -> anyhow::Result<()> {
        let file_name = self.cache_file_name(date);
        let path = self.dir.join(&file_name);
        let content = serde_json::to_string_pretty(data)?;
        tokio::fs::write(&path, content).await?;
        Ok(())
    }

    pub async fn clear(&self) -> anyhow::Result<()> {
        if !self.dir.exists() {
            return Ok(());
        }

        let mut entries = tokio::fs::read_dir(&self.dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                tokio::fs::remove_file(path).await?;
            }
        }

        Ok(())
    }

    pub async fn clear_group(&self, group_id: u32) -> anyhow::Result<()> {
        if !self.dir.exists() {
            return Ok(());
        }

        let prefix = format!("{}-", group_id);
        let mut entries = tokio::fs::read_dir(&self.dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if let Some(name) = path.file_stem().and_then(|s| s.to_str())
                && name.starts_with(&prefix) {
                    tokio::fs::remove_file(path).await?;
                }
        }

        Ok(())
    }
}
