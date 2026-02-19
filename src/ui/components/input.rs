use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub struct InputField<'a> {
    label: &'a str,
    value: &'a str,
    is_active: bool,
}

impl<'a> InputField<'a> {
    pub fn new(label: &'a str, value: &'a str, is_active: bool) -> Self {
        Self {
            label,
            value,
            is_active,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, highlight_color: Color) {
        let style = if self.is_active {
            Style::default()
                .fg(highlight_color)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let display = format!("{}: {}█", self.label, self.value);
        let paragraph = Paragraph::new(display)
            .style(style)
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(paragraph, area);
    }
}
