use crate::app::App;
use osars::models::{Campus, College, Group};
use ratatui::{prelude::*, widgets::*};

pub enum SelectionStage {
    College,
    Campus,
    Group,
}

pub struct SelectorState {
    pub stage: SelectionStage,
    pub colleges: Vec<College>,
    pub campuses: Vec<Campus>,
    pub groups: Vec<Group>,
    pub selected_index: usize,
    pub selected_college: Option<College>,
    pub selected_campus: Option<Campus>,
    pub error_message: Option<String>,
}

impl SelectorState {
    pub fn new() -> Self {
        Self {
            stage: SelectionStage::College,
            colleges: Vec::new(),
            campuses: Vec::new(),
            groups: Vec::new(),
            selected_index: 0,
            selected_college: None,
            selected_campus: None,
            error_message: None,
        }
    }

    pub fn current_items_count(&self) -> usize {
        match self.stage {
            SelectionStage::College => self.colleges.len(),
            SelectionStage::Campus => self.campuses.len(),
            SelectionStage::Group => self.groups.len(),
        }
    }

    pub fn next_item(&mut self) {
        let count = self.current_items_count();
        if count > 0 {
            self.selected_index = (self.selected_index + 1) % count;
        }
    }

    pub fn prev_item(&mut self) {
        let count = self.current_items_count();
        if count > 0 {
            self.selected_index = if self.selected_index == 0 {
                count - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    pub fn get_selected_college(&self) -> Option<&College> {
        self.colleges.get(self.selected_index)
    }

    pub fn get_selected_campus(&self) -> Option<&Campus> {
        self.campuses.get(self.selected_index)
    }

    pub fn get_selected_group(&self) -> Option<&Group> {
        self.groups.get(self.selected_index)
    }
}

pub fn render_selector(f: &mut Frame, app: &App, state: &SelectorState) {
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

    let title = match state.stage {
        SelectionStage::College => "Выберите колледж",
        SelectionStage::Campus => "Выберите кампус",
        SelectionStage::Group => "Выберите группу",
    };

    let header = Paragraph::new(title)
        .style(Style::default().fg(theme.color("highlight")))
        .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);

    let list_area = chunks[1];
    let items: Vec<ListItem> = match state.stage {
        SelectionStage::College => state
            .colleges
            .iter()
            .enumerate()
            .map(|(i, college)| {
                let style = if i == state.selected_index {
                    Style::default().fg(theme.color("highlight"))
                } else {
                    Style::default()
                };
                ListItem::new(format!("{} (ID: {})", college.name, college.college_id)).style(style)
            })
            .collect(),
        SelectionStage::Campus => state
            .campuses
            .iter()
            .enumerate()
            .map(|(i, campus)| {
                let style = if i == state.selected_index {
                    Style::default().fg(theme.color("highlight"))
                } else {
                    Style::default()
                };
                ListItem::new(format!("{} (ID: {})", campus.name, campus.id)).style(style)
            })
            .collect(),
        SelectionStage::Group => state
            .groups
            .iter()
            .enumerate()
            .map(|(i, group)| {
                let style = if i == state.selected_index {
                    Style::default().fg(theme.color("highlight"))
                } else {
                    Style::default()
                };
                ListItem::new(format!("{} (ID: {})", group.name, group.id)).style(style)
            })
            .collect(),
    };

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_index));

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().bg(theme.color("header_bg")));

    f.render_stateful_widget(list, list_area, &mut list_state);

    let info_text = match state.stage {
        SelectionStage::College => "Выберите колледж из списка".to_string(),
        SelectionStage::Campus => format!(
            "Колледж: {}",
            state
                .selected_college
                .as_ref()
                .map_or("Не выбран", |c| &c.name)
        ),
        SelectionStage::Group => format!(
            "Колледж: {}, Кампус: {}",
            state
                .selected_college
                .as_ref()
                .map_or("Не выбран", |c| &c.name),
            state
                .selected_campus
                .as_ref()
                .map_or("Не выбран", |c| &c.name)
        ),
    };

    let info = Paragraph::new(info_text)
        .style(Style::default().fg(theme.color("table_header")))
        .alignment(Alignment::Center);
    f.render_widget(info, chunks[2]);

    if let Some(error) = &state.error_message {
        let error_area = Rect {
            x: area.x,
            y: area.y + area.height - 3,
            width: area.width,
            height: 3,
        };
        let error_widget = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(error_widget, error_area);
    }
}
