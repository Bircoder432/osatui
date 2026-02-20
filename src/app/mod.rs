pub mod events;
pub mod state;

use crate::{api::ApiClient, config::Config};
use crossterm::event::KeyEvent;
pub use state::{AppMode, AppState, SetupField};

pub struct App {
    pub config: Config,
    pub state: AppState,
    api: Option<ApiClient>,
    should_quit: bool,
}

impl App {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        log::debug!("Loading application");
        let mut state = AppState::new(config.clone());

        match ApiClient::new(config.clone()).await {
            Ok(api) => {
                state.load_schedules(&api).await?;
                log::info!("Schedules loaded succesful");
                Ok(Self {
                    config,
                    state,
                    api: Some(api),
                    should_quit: false,
                })
            }
            Err(e) => {
                log::warn!("Failed to initialize API client: {}", e);
                let mut api = ApiClient::new_base(config.clone()).await?;
                state.enter_selector(crate::ui::screens::selector::SelectionStage::College);
                state.load_colleges(&mut api).await?;
                Ok(Self {
                    config,
                    state,
                    api: Some(api),
                    should_quit: false,
                })
            }
        }
    }

    pub async fn handle_key_event(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        use crate::app::events::handle_event;
        handle_event(self, key).await
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut AppState {
        &mut self.state
    }

    pub fn api(&self) -> Option<&ApiClient> {
        self.api.as_ref()
    }

    pub fn api_mut(&mut self) -> Option<&mut ApiClient> {
        self.api.as_mut()
    }

    pub fn take_api(&mut self) -> Option<ApiClient> {
        self.api.take()
    }

    pub fn set_api(&mut self, api: ApiClient) {
        self.api = Some(api);
    }

    pub fn set_error_message(&mut self, msg: String) {
        self.state.set_error_message(msg);
    }

    pub async fn reload_api(&mut self) -> anyhow::Result<()> {
        if let Some(api) = &mut self.api {
            api.reconfigure(&self.config).await?;
            self.state.load_schedules(api).await?;
        } else {
            self.api = Some(ApiClient::new(self.config.clone()).await?);
        }
        Ok(())
    }
}
