use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(area);

    render_schedule_table(f, app, chunks[0]);
    render_help(f, app, chunks[1]);
}

fn render_schedule_table(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();

    if app.schedules().is_empty() {
        let msg = Paragraph::new("Нет занятий на выбранную дату")
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

fn render_help(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();

    let help_text = "F1: предыдущий день | F2: сегодня | F3: следующий день | Ctrl+O: выбор группы | Ctrl+S: настройки | Q: выход";

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(theme.color("table_header")))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(theme.color("border"))),
        );

    f.render_widget(help, area);
}
