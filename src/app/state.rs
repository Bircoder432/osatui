use crate::{api::ApiClient, config::Config, utils::AppDate};
use osars::models::{Campus, College, Group, Schedule};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Normal,
    Selector,
    Setup,
}

pub struct AppState {
    pub mode: AppMode,
    pub config: Config,

    // Normal mode state
    pub current_date: AppDate,
    pub schedules: Vec<Schedule>,

    // Selector mode state
    pub selection_stage: crate::ui::screens::selector::SelectionStage,
    pub colleges: Vec<College>,
    pub campuses: Vec<Campus>,
    pub groups: Vec<Group>,
    pub selected_index: usize,
    pub page: usize,
    pub page_size: usize,
    pub selected_college: Option<College>,
    pub selected_campus: Option<Campus>,

    // Setup mode state
    pub setup_field: SetupField,
    pub setup_api_url: String,
    pub setup_college_id: String,
    pub setup_campus_id: String,
    pub setup_group_name: String,

    // Error display
    pub error_message: Option<String>,
    pub error_timeout: Option<std::time::Instant>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupField {
    ApiUrl,
    CollegeId,
    CampusId,
    GroupName,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            mode: AppMode::Normal,
            config,
            current_date: AppDate::today(),
            schedules: Vec::new(),
            selection_stage: crate::ui::screens::selector::SelectionStage::College,
            colleges: Vec::new(),
            campuses: Vec::new(),
            groups: Vec::new(),
            selected_index: 0,
            page: 0,
            page_size: 20,
            selected_college: None,
            selected_campus: None,
            setup_field: SetupField::ApiUrl,
            setup_api_url: String::new(),
            setup_college_id: String::new(),
            setup_campus_id: String::new(),
            setup_group_name: String::new(),
            error_message: None,
            error_timeout: None,
        }
    }

    pub async fn load_schedules(&mut self, api: &ApiClient) -> anyhow::Result<()> {
        self.schedules = api.fetch(&self.current_date).await?;
        Ok(())
    }

    pub async fn load_colleges(&mut self, api: &mut ApiClient) -> anyhow::Result<()> {
        self.colleges = api.get_colleges().await?;
        self.selected_index = 0;
        self.page = 0;
        Ok(())
    }

    pub async fn load_campuses(
        &mut self,
        api: &mut ApiClient,
        college_id: u32,
    ) -> anyhow::Result<()> {
        self.campuses = api.get_campuses(college_id).await?;
        self.selected_index = 0;
        self.page = 0;
        Ok(())
    }

    pub async fn load_groups(&mut self, api: &mut ApiClient, campus_id: u32) -> anyhow::Result<()> {
        self.groups = api.get_groups(campus_id).await?;
        self.selected_index = 0;
        self.page = 0;
        Ok(())
    }

    pub fn enter_selector(&mut self, stage: crate::ui::screens::selector::SelectionStage) {
        self.mode = AppMode::Selector;
        self.selection_stage = stage;
        self.selected_index = 0;
        self.page = 0;
        self.selected_college = None;
        self.selected_campus = None;
    }

    pub fn enter_setup(&mut self) {
        self.mode = AppMode::Setup;
        self.setup_field = SetupField::ApiUrl;
        self.setup_api_url = self.config.api_url().to_string();
        self.setup_college_id = self.config.college_id().to_string();
        self.setup_campus_id = self.config.campus_id().to_string();
        self.setup_group_name = self.config.group_name().unwrap_or("").to_string();
    }

    pub fn enter_normal(&mut self) {
        self.mode = AppMode::Normal;
        self.error_message = None;
    }

    pub fn set_error_message(&mut self, msg: String) {
        self.error_message = Some(msg);
        self.error_timeout = Some(std::time::Instant::now() + std::time::Duration::from_secs(5));
    }

    pub fn clear_error_if_expired(&mut self) {
        if let Some(timeout) = self.error_timeout
            && std::time::Instant::now() > timeout {
                self.error_message = None;
                self.error_timeout = None;
            }
    }

    // Navigation helpers
    pub fn next_item(&mut self) {
        let count = self.current_items_count();
        if count == 0 {
            return;
        }

        let visible_count = std::cmp::min(self.page_size, count - self.page * self.page_size);

        if self.selected_index + 1 >= visible_count {
            // Move to next page if possible
            let total_pages = count.div_ceil(self.page_size);
            if self.page + 1 < total_pages {
                self.page += 1;
                self.selected_index = 0;
            }
        } else {
            self.selected_index += 1;
        }
    }

    pub fn prev_item(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else if self.page > 0 {
            self.page -= 1;
            let count = self.current_items_count();
            let visible = std::cmp::min(self.page_size, count - self.page * self.page_size);
            self.selected_index = visible.saturating_sub(1);
        }
    }

    pub fn next_page(&mut self) {
        let count = self.current_items_count();
        let total_pages = count.div_ceil(self.page_size);
        if self.page + 1 < total_pages {
            self.page += 1;
            self.selected_index = 0;
        }
    }

    pub fn prev_page(&mut self) {
        if self.page > 0 {
            self.page -= 1;
            self.selected_index = 0;
        }
    }

    fn current_items_count(&self) -> usize {
        use crate::ui::screens::selector::SelectionStage;
        match self.selection_stage {
            SelectionStage::College => self.colleges.len(),
            SelectionStage::Campus => self.campuses.len(),
            SelectionStage::Group => self.groups.len(),
        }
    }

    pub fn get_selected_college(&self) -> Option<&College> {
        let idx = self.page * self.page_size + self.selected_index;
        self.colleges.get(idx)
    }

    pub fn get_selected_campus(&self) -> Option<&Campus> {
        let idx = self.page * self.page_size + self.selected_index;
        self.campuses.get(idx)
    }

    pub fn get_selected_group(&self) -> Option<&Group> {
        let idx = self.page * self.page_size + self.selected_index;
        self.groups.get(idx)
    }

    // Date navigation
    pub fn prev_day(&mut self) {
        self.current_date = self.current_date.prev();
    }

    pub fn next_day(&mut self) {
        self.current_date = self.current_date.next();
    }

    pub fn go_today(&mut self) {
        self.current_date = AppDate::today();
    }
}
