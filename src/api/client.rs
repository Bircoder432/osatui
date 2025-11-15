use crate::{cache::CacheManager, config::Config, utils::date::AppDate};
use osars::{Client, models::Schedule};

pub struct ApiClient {
    client: Client,
    group_id: u32,
    cache: Option<CacheManager>,
}

impl ApiClient {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        let cache = if config.cache_enabled() {
            Some(CacheManager::new(config.cache_ttl()).await?)
        } else {
            None
        };
        Ok(Self {
            client: Client::new(config.api_url()).with_college(config.college_id()),
            group_id: config.group_id(),
            cache,
        })
    }

    pub async fn fetch(&self, date: &AppDate) -> anyhow::Result<Vec<Schedule>> {
        if let Some(cache) = &self.cache {
            if let Some(data) = cache.get(date).await? {
                return Ok(data);
            }
        }

        let schedules = self
            .client
            .schedule(self.group_id)
            .date(&date.iso())
            .send()
            .await?;

        if let Some(cache) = &self.cache {
            let _ = cache.set(date, &schedules).await;
        }

        Ok(schedules)
    }
}
