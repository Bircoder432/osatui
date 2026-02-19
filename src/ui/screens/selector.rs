use crate::app::{App, AppState};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionStage {
    College,
    Campus,
    Group,
}

pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();
    let state = app.state();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(area);

    render_title(f, state, chunks[0]);
    render_list(f, state, chunks[1]);
    render_info(f, state, chunks[2]);
    render_pagination(f, state, chunks[3]);
    render_help(f, chunks[4]);
}

fn render_title(f: &mut Frame, state: &AppState, area: Rect) {
    let theme = state.config.theme();

    let title = match state.selection_stage {
        SelectionStage::College => "Select College",
        SelectionStage::Campus => "Select Campus",
        SelectionStage::Group => "Select Group",
    };

    let header = Paragraph::new(title)
        .style(Style::default().fg(theme.highlight_color()))
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(header, area);
}

fn render_list(f: &mut Frame, state: &AppState, area: Rect) {
    let theme = state.config.theme();

    let items: Vec<ListItem> = match state.selection_stage {
        SelectionStage::College => state
            .colleges
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let style = if i == state.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{} (ID: {})", c.name, c.college_id)).style(style)
            })
            .collect(),
        SelectionStage::Campus => state
            .campuses
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let style = if i == state.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{} (ID: {})", c.name, c.id)).style(style)
            })
            .collect(),
        SelectionStage::Group => state
            .groups
            .iter()
            .enumerate()
            .map(|(i, g)| {
                let style = if i == state.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{} (ID: {})", g.name, g.id)).style(style)
            })
            .collect(),
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().bg(theme.header_bg_color()));

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_index));

    f.render_stateful_widget(list, area, &mut list_state);
}

fn render_info(f: &mut Frame, state: &AppState, area: Rect) {
    let theme = state.config.theme();

    let info_text = match state.selection_stage {
        SelectionStage::College => "Select a college from the list".to_string(),
        SelectionStage::Campus => format!(
            "College: {}",
            state
                .selected_college
                .as_ref()
                .map_or("Not selected", |c| &c.name)
        ),
        SelectionStage::Group => format!(
            "College: {}, Campus: {}",
            state
                .selected_college
                .as_ref()
                .map_or("Not selected", |c| &c.name),
            state
                .selected_campus
                .as_ref()
                .map_or("Not selected", |c| &c.name)
        ),
    };

    let info = Paragraph::new(info_text)
        .style(Style::default().fg(theme.table_header_color()))
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(info, area);
}

fn render_pagination(f: &mut Frame, state: &AppState, area: Rect) {
    let theme = state.config.theme();

    let total_items = match state.selection_stage {
        SelectionStage::College => state.colleges.len(),
        SelectionStage::Campus => state.campuses.len(),
        SelectionStage::Group => state.groups.len(),
    };

    let total_pages = (total_items + state.page_size - 1) / state.page_size.max(1);

    let pagination_text = if total_pages > 1 {
        format!(
            "Page {}/{} ({} items) ←→",
            state.page + 1,
            total_pages,
            total_items
        )
    } else {
        format!("Total items: {}", total_items)
    };

    let pagination = Paragraph::new(pagination_text)
        .style(Style::default().fg(theme.table_header_color()))
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(pagination, area);
}

fn render_help(f: &mut Frame, area: Rect) {
    let help_text = "↑↓: navigate | ←→: pages | Enter: select | Esc: cancel";
    let help = Paragraph::new(help_text)
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::TOP));

    f.render_widget(help, area);
}
