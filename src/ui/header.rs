use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let date_str = app.date().format();

    let college_info = if let Some(college_name) = app.config.college_name() {
        format!("Колледж: {}", college_name)
    } else {
        format!("Колледж ID: {}", app.config.college_id())
    };

    let campus_info = if let Some(campus_name) = app.config.campus_name() {
        format!("Кампус: {}", campus_name)
    } else {
        format!("Кампус ID: {}", app.config.campus_id())
    };

    let group_info = if let Some(group_name) = app.config.group_name() {
        format!("Группа: {}", group_name)
    } else {
        format!("Группа ID: {}", app.config.group_id())
    };

    let schedule_info = if app.schedules().is_empty() {
        "Нет занятий".to_string()
    } else {
        let lessons_count = app
            .schedules()
            .iter()
            .map(|s| s.lessons.len())
            .sum::<usize>();
        format!("{} занятий", lessons_count)
    };

    let header_text = format!(
        " {} | {} | {} | {} | {} ",
        date_str, college_info, campus_info, group_info, schedule_info
    );

    let header = Paragraph::new(header_text)
        .style(
            Style::default()
                .bg(theme.color("header_bg"))
                .fg(theme.color("header_fg")),
        )
        .alignment(Alignment::Center);

    f.render_widget(header, area);
}
