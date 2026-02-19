pub mod components;
pub mod screens;

use crate::app::{App, AppMode};
use ratatui::prelude::*;

pub fn render(f: &mut Frame, app: &App) {
    match app.state().mode {
        AppMode::Normal => screens::normal::render(f, app),
        AppMode::Selector => screens::selector::render(f, app),
        AppMode::Setup => screens::setup::render(f, app),
    }

    if let Some(ref error) = app.state().error_message {
        render_error_popup(f, error);
    }
}

fn render_error_popup(f: &mut Frame, error: &str) {
    let area = f.area();
    let popup_area = Rect {
        x: area.width / 4,
        y: area.height / 2 - 2,
        width: area.width / 2,
        height: 4,
    };

    let block = ratatui::widgets::Block::default()
        .title("Error")
        .borders(ratatui::widgets::Borders::ALL)
        .border_style(Style::default().fg(Color::Red));

    let paragraph = ratatui::widgets::Paragraph::new(error)
        .style(Style::default().fg(Color::Red))
        .wrap(ratatui::widgets::Wrap { trim: true })
        .block(block);

    f.render_widget(paragraph, popup_area);
}
