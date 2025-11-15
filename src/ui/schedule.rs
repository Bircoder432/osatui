use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();

    if app.schedules().is_empty() {
        let msg = Paragraph::new("Нет занятий")
            .style(Style::default().fg(theme.color("highlight")))
            .alignment(Alignment::Center);
        f.render_widget(msg, area);
        return;
    }

    let rows: Vec<Row> = app
        .schedules()
        .iter()
        .flat_map(|s| {
            s.lessons.iter().map(|l| {
                Row::new(vec![
                    l.start_time.format("%H:%M").to_string(),
                    l.end_time.format("%H:%M").to_string(),
                    l.title.clone(),
                    l.cabinet.clone(),
                    l.teacher.clone(),
                ])
            })
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(12),
            Constraint::Percentage(12),
            Constraint::Percentage(45),
            Constraint::Percentage(15),
            Constraint::Percentage(16),
        ],
    )
    .header(
        Row::new(vec!["Нач", "Кон", "Предмет", "Каб", "Преп"])
            .style(Style::default().fg(theme.color("table_header"))),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.color("border")))
            .title("Расписание"),
    );

    f.render_widget(table, area);
}
