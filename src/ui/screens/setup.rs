use crate::app::{App, state::SetupField};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();
    let state = app.state();
    let theme = state.config.theme();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(area);

    let header = Paragraph::new("Application Settings")
        .style(Style::default().fg(theme.highlight_color()))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(header, chunks[0]);

    let fields_area = chunks[1];
    let field_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(fields_area);

    let fields = [
        (SetupField::ApiUrl, "API URL", &state.setup_api_url),
        (SetupField::CollegeId, "College ID", &state.setup_college_id),
        (SetupField::CampusId, "Campus ID", &state.setup_campus_id),
        (SetupField::GroupName, "Group Name", &state.setup_group_name),
    ];

    for (i, (field, label, value)) in fields.iter().enumerate() {
        let is_active = *field == state.setup_field;
        let style = if is_active {
            Style::default()
                .fg(theme.highlight_color())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let display_value = if is_active {
            format!("{}: {}█", label, value)
        } else {
            format!("{}: {}", label, value)
        };

        let input = Paragraph::new(display_value)
            .style(style)
            .block(Block::default().borders(Borders::ALL));

        f.render_widget(input, field_chunks[i]);
    }

    let help = Paragraph::new("Enter: save | Tab: next field | Shift+Tab: prev field | Esc: exit")
        .style(Style::default().fg(theme.table_header_color()))
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(help, chunks[2]);
}
