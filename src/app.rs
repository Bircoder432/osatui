use crate::{
    api::ApiClient,
    cache::{self, CacheManager},
    config::Config,
    ui::{
        selector::{SelectionStage, SelectorState},
        setup::SetupState,
    },
    utils::AppDate,
};
use crossterm::event::KeyCode;
use osars::models::Schedule;

pub enum AppMode {
    Normal,
    Setup(SetupState),
    Selector(SelectorState),
}

pub struct App {
    pub config: Config,
    api: Option<ApiClient>,
    schedules: Vec<Schedule>,
    date: AppDate,
    should_quit: bool,
    mode: AppMode,
}

impl App {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        match ApiClient::new(config.clone()).await {
            Ok(mut api) => {
                let date = AppDate::today();
                let mut schedules = api.fetch(&date).await?;
                if schedules.get(0).unwrap_or_default().group_id != config.group_id() {
                    api.clear_cache().await?;
                    schedules = api.fetch(&date).await?;
                }
                Ok(Self {
                    config,
                    api: Some(api),
                    schedules,
                    date,
                    should_quit: false,
                    mode: AppMode::Normal,
                })
            }
            Err(e) => {
                eprintln!("Ошибка инициализации: {}. Запуск селектора...", e);
                let base_client = ApiClient::new_base(config.clone()).await?;
                let mut app = Self {
                    config,
                    api: Some(base_client),
                    schedules: Vec::new(),
                    date: AppDate::today(),
                    should_quit: false,
                    mode: AppMode::Selector(SelectorState::new()),
                };
                app.load_colleges().await?;
                Ok(app)
            }
        }
    }

    async fn load_colleges(&mut self) -> anyhow::Result<()> {
        if let (AppMode::Selector(selector), Some(api)) = (&mut self.mode, &mut self.api) {
            match api.get_colleges().await {
                Ok(colleges) => {
                    selector.colleges = colleges;
                    if selector.colleges.is_empty() {
                        selector.error_message =
                            Some("Список колледжей пуст. Проверьте URL API.".to_string());
                    }
                }
                Err(e) => {
                    selector.error_message = Some(format!("Ошибка загрузки колледжей: {}", e));
                }
            }
        }
        Ok(())
    }

    async fn load_campuses(&mut self, college_id: u32) -> anyhow::Result<()> {
        if let (AppMode::Selector(selector), Some(api)) = (&mut self.mode, &mut self.api) {
            match api.get_campuses(college_id).await {
                Ok(campuses) => {
                    selector.campuses = campuses;
                    if selector.campuses.is_empty() {
                        selector.error_message = Some("Список кампусов пуст.".to_string());
                    }
                }
                Err(e) => {
                    selector.error_message = Some(format!("Ошибка загрузки кампусов: {}", e));
                }
            }
        }
        Ok(())
    }

    async fn load_groups(&mut self, campus_id: u32) -> anyhow::Result<()> {
        if let (AppMode::Selector(selector), Some(api)) = (&mut self.mode, &mut self.api) {
            match api.get_groups(campus_id).await {
                Ok(groups) => {
                    selector.groups = groups;
                    if selector.groups.is_empty() {
                        selector.error_message = Some("Список групп пуст.".to_string());
                    }
                }
                Err(e) => {
                    selector.error_message = Some(format!("Ошибка загрузки групп: {}", e));
                }
            }
        }
        Ok(())
    }

    pub async fn reload_cache(&mut self) -> anyhow::Result<()> {
        if let Some(api) = &mut self.api {
            api.clear_cache().await?;

            self.schedules = api.fetch(&self.date).await?;
            println!("Кеш успешно очищен и перезагружен");
        }
        Ok(())
    }

    pub fn start_setup(&mut self) {
        let mut state = SetupState::new();
        state.api_url = self.config.api_url().to_string();
        state.college_id = self.config.college_id().to_string();
        state.campus_id = self.config.campus_id().to_string();
        state.group_name = self.config.group_name().unwrap_or("").to_string();
        self.mode = AppMode::Setup(state);
    }

    pub async fn start_selector(&mut self) -> anyhow::Result<()> {
        let api = ApiClient::new_base(self.config.clone()).await?;
        self.api = Some(api);
        self.mode = AppMode::Selector(SelectorState::new());
        self.load_colleges().await?;
        Ok(())
    }

    pub async fn handle_selector_input(&mut self) -> anyhow::Result<()> {
        if let AppMode::Selector(selector) = &mut self.mode {
            match selector.stage {
                SelectionStage::College => {
                    if let Some(college) = selector.get_selected_college().cloned() {
                        let college_id = college.college_id;
                        selector.selected_college = Some(college);
                        selector.stage = SelectionStage::Campus;
                        selector.reset_pagination();
                        selector.error_message = None;
                        self.load_campuses(college_id).await?;
                    }
                }
                SelectionStage::Campus => {
                    if let Some(campus) = selector.get_selected_campus().cloned() {
                        let campus_id = campus.id;
                        selector.selected_campus = Some(campus);
                        selector.stage = SelectionStage::Group;
                        selector.reset_pagination();
                        selector.error_message = None;
                        self.load_groups(campus_id).await?;
                    }
                }
                SelectionStage::Group => {
                    if let Some(group) = selector.get_selected_group() {
                        if let Some(college) = &selector.selected_college {
                            self.config
                                .set_college(college.college_id, Some(college.name.clone()));
                        }
                        if let Some(campus) = &selector.selected_campus {
                            self.config.set_campus(campus.id, Some(campus.name.clone()));
                        }
                        self.config.set_group(group.id, Some(group.name.clone()));
                        self.config.save().await?;
                        self.api = Some(ApiClient::new(self.config.clone()).await?);
                        self.schedules = self.api.as_ref().unwrap().fetch(&self.date).await?;
                        self.mode = AppMode::Normal;
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn handle_selector_navigation(&mut self, key: KeyCode) {
        if let AppMode::Selector(selector) = &mut self.mode {
            match key {
                KeyCode::Down => selector.next_item(),
                KeyCode::Up => selector.prev_item(),
                KeyCode::Right => selector.next_page(),
                KeyCode::Left => selector.prev_page(),
                _ => {}
            }
        }
    }

    pub async fn handle_setup_input(&mut self, _input: char) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn prev_day(&mut self) -> anyhow::Result<()> {
        if let (AppMode::Normal, Some(api)) = (&self.mode, &self.api) {
            self.date = self.date.prev();
            self.schedules = api.fetch(&self.date).await?;
        }
        Ok(())
    }

    pub async fn next_day(&mut self) -> anyhow::Result<()> {
        if let (AppMode::Normal, Some(api)) = (&self.mode, &self.api) {
            self.date = self.date.next();
            self.schedules = api.fetch(&self.date).await?;
        }
        Ok(())
    }

    pub async fn go_today(&mut self) -> anyhow::Result<()> {
        if let (AppMode::Normal, Some(api)) = (&self.mode, &self.api) {
            self.date = AppDate::today();
            self.schedules = api.fetch(&self.date).await?;
        }
        Ok(())
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn date(&self) -> &AppDate {
        &self.date
    }

    pub fn schedules(&self) -> &[Schedule] {
        &self.schedules
    }

    pub fn theme(&self) -> &crate::config::theme::Theme {
        self.config.theme()
    }

    pub fn mode(&self) -> &AppMode {
        &self.mode
    }

    pub fn mode_mut(&mut self) -> &mut AppMode {
        &mut self.mode
    }
}
