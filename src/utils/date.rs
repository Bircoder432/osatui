use chrono::{Duration, NaiveDate, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppDate(NaiveDate);

impl AppDate {
    pub fn today() -> Self {
        Self(Utc::now().date_naive())
    }

    pub fn prev(self) -> Self {
        Self(self.0 - Duration::days(1))
    }

    pub fn next(self) -> Self {
        Self(self.0 + Duration::days(1))
    }

    pub fn format(&self) -> String {
        let today = Utc::now().date_naive();
        if self.0 == today {
            "Today".into()
        } else if self.0 == today + Duration::days(1) {
            "Tomorrow".into()
        } else if self.0 == today - Duration::days(1) {
            "Yesterday".into()
        } else {
            self.0.format("%d %B %Y").to_string()
        }
    }

    pub fn iso(&self) -> String {
        self.0.format("%d-%m-%Y").to_string()
    }
}

impl From<NaiveDate> for AppDate {
    fn from(date: NaiveDate) -> Self {
        Self(date)
    }
}

impl From<AppDate> for NaiveDate {
    fn from(date: AppDate) -> Self {
        date.0
    }
}
