use crate::app::App;
use ratatui::{prelude::*, widgets::*};

#[derive(Clone)]
pub struct SetupState {
    pub current_field: usize,
    pub api_url: String,
    pub college_id: String,
    pub campus_id: String,
    pub group_name: String,
    pub error_message: Option<String>,
}

impl SetupState {
    pub fn new() -> Self {
        Self {
            current_field: 0,
            api_url: String::new(),
            college_id: String::new(),
            campus_id: String::new(),
            group_name: String::new(),
            error_message: None,
        }
    }

    pub fn fields_count() -> usize {
        4
    }

    pub fn next_field(&mut self) {
        self.current_field = (self.current_field + 1) % Self::fields_count();
    }

    pub fn prev_field(&mut self) {
        self.current_field = if self.current_field == 0 {
            Self::fields_count() - 1
        } else {
            self.current_field - 1
        };
    }

    pub fn current_field_mut(&mut self) -> &mut String {
        match self.current_field {
            0 => &mut self.api_url,
            1 => &mut self.college_id,
            2 => &mut self.campus_id,
            3 => &mut self.group_name,
            _ => &mut self.api_url,
        }
    }
}

pub fn render_setup(f: &mut Frame, app: &App, state: &SetupState) {
    let theme = app.theme();
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(area);

    let header = Paragraph::new("Настройка приложения")
        .style(Style::default().fg(theme.color("highlight")))
        .alignment(Alignment::Center);
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
        ("API URL", &state.api_url),
        ("College ID", &state.college_id),
        ("Campus ID", &state.campus_id),
        ("Group Name", &state.group_name),
    ];

    for (i, (label, value)) in fields.iter().enumerate() {
        let is_active = i == state.current_field;
        let style = if is_active {
            Style::default().fg(theme.color("highlight"))
        } else {
            Style::default()
        };

        let input = Paragraph::new(format!("{}: {}", label, value))
            .style(style)
            .block(Block::default().borders(Borders::ALL));

        f.render_widget(input, field_chunks[i]);

        if is_active {
            f.set_cursor_position(Position::new(
                field_chunks[i].x + label.len() as u16 + 2 + value.len() as u16 + 1,
                field_chunks[i].y + 1,
            ));
        }
    }

    if let Some(error) = &state.error_message {
        let error_widget = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center);
        f.render_widget(error_widget, field_chunks[4]);
    }

    let help = Paragraph::new(
        "Enter: сохранить | Tab: следующее поле | Shift+Tab: предыдущее поле | Esc: выйти",
    )
    .style(Style::default().fg(theme.color("table_header")))
    .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}
