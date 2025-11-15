use chrono::{Duration, NaiveDate, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppDate(NaiveDate);

impl AppDate {
    pub fn today() -> Self {
        Self(Utc::today().naive_utc())
    }
    pub fn prev(self) -> Self {
        Self(self.0 - Duration::days(1))
    }
    pub fn next(self) -> Self {
        Self(self.0 + Duration::days(1))
    }
    pub fn format(&self) -> String {
        let today = Utc::today().naive_utc();
        if self.0 == today {
            "Сегодня".into()
        } else if self.0 == today + Duration::days(1) {
            "Завтра".into()
        } else if self.0 == today - Duration::days(1) {
            "Вчера".into()
        } else {
            self.0.format("%d %B %Y").to_string()
        }
    }
    pub fn iso(&self) -> String {
        self.0.format("%d-%m-%Y").to_string() // dd-MM-yyyy
    }
}
