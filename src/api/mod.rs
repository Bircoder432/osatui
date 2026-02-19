pub mod cache;

use crate::{config::Config, utils::AppDate};
use cache::CacheManager;
use osars::{
    Client,
    models::{Campus, College, Group, Schedule},
};
use std::collections::HashMap;

pub struct ApiClient {
    config: Config,
    client: Client,
    college_id: Option<u32>,
    campus_id: Option<u32>,
    group_id: Option<u32>,
    cache: Option<CacheManager>,
    lists_cache: HashMap<String, (Vec<u8>, u64)>,
    lists_cache_ttl: u64,
}

impl ApiClient {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let mut cache = if config.cache_enabled() {
            Some(CacheManager::new(config.cache_ttl()).await?)
        } else {
            None
        };

        let client = Client::new(config.api_url());
        let (college_id, campus_id, group_id) = Self::resolve_ids(&client, &config).await?;

        if let Some(ref mut c) = cache {
            c.set_group_id(group_id);
        }

        Ok(Self {
            config,
            client: client.with_college(college_id),
            college_id: Some(college_id),
            campus_id: Some(campus_id),
            group_id: Some(group_id),
            cache,
            lists_cache: HashMap::new(),
            lists_cache_ttl: 300,
        })
    }

    pub async fn new_base(config: Config) -> anyhow::Result<Self> {
        let cache = if config.cache_enabled() {
            Some(CacheManager::new(config.cache_ttl()).await?)
        } else {
            None
        };

        Ok(Self {
            config: config.clone(),
            client: Client::new(config.api_url()),
            college_id: None,
            campus_id: None,
            group_id: None,
            cache,
            lists_cache: HashMap::new(),
            lists_cache_ttl: 300,
        })
    }

    pub async fn reconfigure(&mut self, config: &Config) -> anyhow::Result<()> {
        let old_group_id = self.group_id;

        self.config = config.clone();
        self.client = Client::new(config.api_url());

        if let Some(cache) = &mut self.cache {
            cache.update_ttl(config.cache_ttl());
        }

        let (college_id, campus_id, group_id) = Self::resolve_ids(&self.client, config).await?;

        if let Some(ref mut c) = self.cache {
            c.set_group_id(group_id);
        }

        self.client = self.client.clone().with_college(college_id);
        self.college_id = Some(college_id);
        self.campus_id = Some(campus_id);
        self.group_id = Some(group_id);

        // Очищаем старый кэш если группа изменилась
        if old_group_id != Some(group_id)
            && let Some(cache) = &self.cache
                && let Some(old_gid) = old_group_id {
                    cache.clear_group(old_gid).await?;
                }

        Ok(())
    }

    async fn resolve_ids(client: &Client, config: &Config) -> anyhow::Result<(u32, u32, u32)> {
        let college_id = if let Some(college_name) = config.college_name() {
            Self::find_college_id(client, college_name).await?
        } else {
            config.college_id()
        };

        let campus_id = if let Some(campus_name) = config.campus_name() {
            Self::find_campus_id(config.api_url(), college_id, campus_name).await?
        } else {
            config.campus_id()
        };

        let group_id = if let Some(group_name) = config.group_name() {
            Self::find_group_id(client, campus_id, group_name).await?
        } else {
            config.group_id()
        };

        Ok((college_id, campus_id, group_id))
    }

    async fn find_college_id(client: &Client, college_name: &str) -> anyhow::Result<u32> {
        let colleges: Vec<College> = client.colleges().name(college_name).send().await?;
        colleges
            .first()
            .map(|c| c.college_id)
            .ok_or_else(|| anyhow::anyhow!("College '{}' not found", college_name))
    }

    async fn find_campus_id(
        api_url: &str,
        college_id: u32,
        campus_name: &str,
    ) -> anyhow::Result<u32> {
        let client = Client::new(api_url).with_college(college_id);
        let campuses: Vec<Campus> = client.campuses()?.send().await?;

        let campus = campuses
            .into_iter()
            .find(|c| c.name.to_lowercase().contains(&campus_name.to_lowercase()));

        campus.map(|c| c.id).ok_or_else(|| {
            anyhow::anyhow!(
                "Campus '{}' not found in college {}",
                campus_name,
                college_id
            )
        })
    }

    async fn find_group_id(
        client: &Client,
        campus_id: u32,
        group_name: &str,
    ) -> anyhow::Result<u32> {
        let groups: Vec<Group> = client.groups(campus_id).name(group_name).send().await?;
        groups
            .first()
            .map(|g| g.id)
            .ok_or_else(|| anyhow::anyhow!("Group '{}' not found", group_name))
    }

    pub async fn get_colleges(&mut self) -> anyhow::Result<Vec<College>> {
        let api_url = self.config.api_url().to_string();
        self.get_cached_list("colleges", move || async move {
            Client::new(&api_url)
                .colleges()
                .send()
                .await
                .map_err(|e| e.into())
        })
        .await
    }

    pub async fn get_campuses(&mut self, college_id: u32) -> anyhow::Result<Vec<Campus>> {
        let api_url = self.config.api_url().to_string();
        let key = format!("campuses_{}", college_id);
        self.get_cached_list(&key, move || async move {
            let client = Client::new(&api_url).with_college(college_id);
            client.campuses()?.send().await.map_err(|e| e.into())
        })
        .await
    }

    pub async fn get_groups(&mut self, campus_id: u32) -> anyhow::Result<Vec<Group>> {
        let api_url = self.config.api_url().to_string();
        let key = format!("groups_{}", campus_id);
        self.get_cached_list(&key, move || async move {
            Client::new(&api_url)
                .groups(campus_id)
                .send()
                .await
                .map_err(|e| e.into())
        })
        .await
    }

    async fn get_cached_list<F, Fut, T>(&mut self, key: &str, fetch: F) -> anyhow::Result<Vec<T>>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = anyhow::Result<Vec<T>>>,
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        use std::time::{SystemTime, UNIX_EPOCH};

        if let Some((cached_data, timestamp)) = self.lists_cache.get(key) {
            let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            if current_time - timestamp < self.lists_cache_ttl {
                return Ok(serde_json::from_slice(cached_data)?);
            }
        }

        let data = fetch().await?;
        let serialized = serde_json::to_vec(&data)?;
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        self.lists_cache
            .insert(key.to_string(), (serialized, timestamp));

        Ok(data)
    }

    pub async fn fetch(&self, date: &AppDate) -> anyhow::Result<Vec<Schedule>> {
        if let Some(cache) = &self.cache
            && let Some(data) = cache.get(date).await? {
                return Ok(data);
            }

        let group_id = self
            .group_id
            .ok_or_else(|| anyhow::anyhow!("Group ID not set"))?;

        let result: Result<Vec<Schedule>, _> = self
            .client
            .schedule(group_id)
            .date(&date.iso())
            .send()
            .await;

        let schedules = match result {
            Ok(s) => s,
            Err(e) => {
                let err_str = e.to_string();
                if err_str.contains("null") || err_str.contains("expected a sequence") {
                    Vec::new()
                } else {
                    return Err(e.into());
                }
            }
        };

        if let Some(cache) = &self.cache {
            let _ = cache.set(date, &schedules).await;
        }

        Ok(schedules)
    }

    pub async fn clear_cache(&mut self) -> anyhow::Result<()> {
        if let Some(cache) = &self.cache {
            cache.clear().await?;
        }
        self.lists_cache.clear();
        Ok(())
    }

    pub async fn clear_current_group_cache(&self) -> anyhow::Result<()> {
        if let (Some(cache), Some(group_id)) = (&self.cache, self.group_id) {
            cache.clear_group(group_id).await?;
        }
        Ok(())
    }
}
