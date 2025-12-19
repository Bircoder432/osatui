use crate::{
    cache::{self, CacheManager},
    config::Config,
    utils::date::AppDate,
};
use osars::{
    Client,
    models::{Campus, College, Group, Schedule},
};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ApiClient {
    config: Config,
    client: Client,
    college_id: Option<u32>,
    campus_id: Option<u32>,
    group_id: Option<u32>,
    cache: Option<CacheManager>,
    lists_cache: HashMap<String, (Vec<u8>, u64)>,
}

impl ApiClient {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let cache = if config.cache_enabled() {
            Some(CacheManager::new(config.cache_ttl()).await?)
        } else {
            None
        };

        let client = Client::new(config.api_url());
        let (college_id, campus_id, group_id) = Self::resolve_ids(&client, &config).await?;

        Ok(Self {
            config,
            client: client.with_college(college_id),
            college_id: Some(college_id),
            campus_id: Some(campus_id),
            group_id: Some(group_id),
            cache,
            lists_cache: HashMap::new(),
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
        })
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
            .ok_or_else(|| anyhow::anyhow!("Колледж с именем '{}' не найден", college_name))
    }

    async fn find_campus_id(
        api_url: &str,
        college_id: u32,
        campus_name: &str,
    ) -> anyhow::Result<u32> {
        let client = Client::new(api_url).with_college(college_id);
        let campuses: Vec<Campus> = client
            .campuses()
            .map_err(|_| anyhow::anyhow!("Не удалось создать запрос кампусов"))?
            .send()
            .await?;

        let campus = campuses
            .into_iter()
            .find(|c| c.name.to_lowercase().contains(&campus_name.to_lowercase()));

        campus.map(|c| c.id).ok_or_else(|| {
            anyhow::anyhow!(
                "Кампус с именем '{}' не найден в колледже {}",
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
            .ok_or_else(|| anyhow::anyhow!("Группа с именем '{}' не найдена", group_name))
    }

    pub async fn get_colleges(&mut self) -> anyhow::Result<Vec<College>> {
        let cache_key = "colleges_list".to_string();
        if let Some((cached_data, timestamp)) = self.lists_cache.get(&cache_key) {
            let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            if current_time - timestamp < 300 {
                return Ok(serde_json::from_slice(cached_data)?);
            }
        }

        let colleges = Client::new(self.config.api_url()).colleges().send().await?;
        let serialized = serde_json::to_vec(&colleges)?;
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        self.lists_cache.insert(cache_key, (serialized, timestamp));

        Ok(colleges)
    }

    pub async fn get_campuses(&mut self, college_id: u32) -> anyhow::Result<Vec<Campus>> {
        let cache_key = format!("campuses_list_{}", college_id);
        if let Some((cached_data, timestamp)) = self.lists_cache.get(&cache_key) {
            let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            if current_time - timestamp < 300 {
                return Ok(serde_json::from_slice(cached_data)?);
            }
        }

        let client = Client::new(self.config.api_url()).with_college(college_id);
        let campuses = client
            .campuses()
            .map_err(|e| anyhow::anyhow!("Не удалось создать запрос кампусов: {}", e))?
            .send()
            .await?;

        let serialized = serde_json::to_vec(&campuses)?;
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        self.lists_cache.insert(cache_key, (serialized, timestamp));

        Ok(campuses)
    }

    pub async fn get_groups(&mut self, campus_id: u32) -> anyhow::Result<Vec<Group>> {
        let cache_key = format!("groups_list_{}", campus_id);
        if let Some((cached_data, timestamp)) = self.lists_cache.get(&cache_key) {
            let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            if current_time - timestamp < 300 {
                return Ok(serde_json::from_slice(cached_data)?);
            }
        }

        let groups = Client::new(self.config.api_url())
            .groups(campus_id)
            .send()
            .await?;
        let serialized = serde_json::to_vec(&groups)?;
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        self.lists_cache.insert(cache_key, (serialized, timestamp));

        Ok(groups)
    }

    pub async fn fetch(&self, date: &AppDate) -> anyhow::Result<Vec<Schedule>> {
        if let Some(cache) = &self.cache {
            if let Some(data) = cache.get(date).await? {
                return Ok(data);
            }
        }

        let group_id = self
            .group_id
            .ok_or_else(|| anyhow::anyhow!("Group ID not set"))?;
        let schedules = self
            .client
            .schedule(group_id)
            .date(&date.iso())
            .send()
            .await?;

        if let Some(cache) = &self.cache {
            let _ = cache.set(date, &schedules).await;
        }

        Ok(schedules)
    }

    pub fn college_id(&self) -> Option<u32> {
        self.college_id
    }

    pub fn campus_id(&self) -> Option<u32> {
        self.campus_id
    }

    pub fn group_id(&self) -> Option<u32> {
        self.group_id
    }

    pub async fn clear_cache(&mut self) -> anyhow::Result<()> {
        let cache = self.cache.as_mut().ok_or("Менеджер кеша не найден");
        match cache {
            Ok(c) => c.clear().await?,
            Err(e) => {
                return Err(anyhow::Error::msg(e));
            }
        }
        self.lists_cache.clear();
        Ok(())
    }
}
