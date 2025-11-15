use crate::{api::ApiClient, config::Config, utils::AppDate};
use osars::models::Schedule;

pub struct App {
    config: Config,
    api: ApiClient,
    schedules: Vec<Schedule>,
    date: AppDate,
    should_quit: bool,
}

impl App {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let api = ApiClient::new(&config).await?;
        let date = AppDate::today();
        let schedules = api.fetch(&date).await?;
        Ok(Self {
            config,
            api,
            schedules,
            date,
            should_quit: false,
        })
    }

    pub async fn prev_day(&mut self) -> anyhow::Result<()> {
        self.date = self.date.prev();
        self.schedules = self.api.fetch(&self.date).await?;
        Ok(())
    }

    pub async fn next_day(&mut self) -> anyhow::Result<()> {
        self.date = self.date.next();
        self.schedules = self.api.fetch(&self.date).await?;
        Ok(())
    }

    pub async fn go_today(&mut self) -> anyhow::Result<()> {
        self.date = AppDate::today();
        self.schedules = self.api.fetch(&self.date).await?;
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
}
