use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    render_header(f, app, chunks[0]);
    render_schedule(f, app, chunks[1]);
    render_footer(f, app, chunks[2]);
}

fn render_header(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let theme = app.state().config.theme();
    let state = app.state();

    let date_str = state.current_date.format();
    let college_info = state
        .config
        .college_name()
        .map(|n| format!("College: {}", n))
        .unwrap_or_else(|| format!("College ID: {}", state.config.college_id()));
    let campus_info = state
        .config
        .campus_name()
        .map(|n| format!("Campus: {}", n))
        .unwrap_or_else(|| format!("Campus ID: {}", state.config.campus_id()));
    let group_info = state
        .config
        .group_name()
        .map(|n| format!("Group: {}", n))
        .unwrap_or_else(|| format!("Group ID: {}", state.config.group_id()));

    let lessons_count: usize = state.schedules.iter().map(|s| s.lessons.len()).sum();
    let schedule_info = if lessons_count == 0 {
        "No lessons".to_string()
    } else {
        format!("{} lessons", lessons_count)
    };

    let header_text = format!(
        " {} | {} | {} | {} | {} ",
        date_str, college_info, campus_info, group_info, schedule_info
    );

    let header = Paragraph::new(header_text)
        .style(
            Style::default()
                .bg(theme.header_bg_color())
                .fg(theme.header_fg_color()),
        )
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(header, area);
}

fn render_schedule(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let theme = app.state().config.theme();
    let state = app.state();

    if state.schedules.is_empty() || state.schedules.iter().all(|s| s.lessons.is_empty()) {
        let msg = Paragraph::new("No lessons for selected date")
            .style(Style::default().fg(theme.highlight_color()))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(msg, area);
        return;
    }

    let rows: Vec<Row> = state
        .schedules
        .iter()
        .flat_map(|s| {
            s.lessons.iter().map(|l| {
                Row::new(vec![
                    Cell::from(l.start_time.format("%H:%M").to_string()),
                    Cell::from(l.end_time.format("%H:%M").to_string()),
                    Cell::from(l.title.clone()),
                    Cell::from(l.cabinet.clone()),
                    Cell::from(l.teacher.clone()),
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
        Row::new(vec!["Start", "End", "Subject", "Room", "Teacher"])
            .style(Style::default().fg(theme.table_header_color())),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border_color()))
            .title("Schedule"),
    );

    f.render_widget(table, area);
}

fn render_footer(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let theme = app.state().config.theme();
    let keymap = app.state().config.keymap();

    let help_text = format!(
        "{}: prev day | {}: today | {}: next day | {}: select group | {}: settings | {}: quit",
        format_key(keymap.prev_day()),
        format_key(keymap.cur_day()),
        format_key(keymap.next_day()),
        format_key(keymap.selector()),
        format_key(keymap.settings()),
        format_key(keymap.exit())
    );

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(theme.table_header_color()))
        .alignment(ratatui::layout::Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(theme.border_color())),
        );

    f.render_widget(help, area);
}

fn format_key(key: crossterm::event::KeyCode) -> String {
    match key {
        crossterm::event::KeyCode::Left => "←".to_string(),
        crossterm::event::KeyCode::Right => "→".to_string(),
        crossterm::event::KeyCode::Up => "↑".to_string(),
        crossterm::event::KeyCode::Down => "↓".to_string(),
        crossterm::event::KeyCode::Char(c) => c.to_string(),
        crossterm::event::KeyCode::F(n) => format!("F{}", n),
        _ => format!("{:?}", key),
    }
}
