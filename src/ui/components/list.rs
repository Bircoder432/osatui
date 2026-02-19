use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};

pub struct SelectableList<'a, T> {
    items: &'a [T],
    title: &'a str,
    selected: usize,
    formatter: fn(&T) -> String,
}

impl<'a, T> SelectableList<'a, T> {
    pub fn new(items: &'a [T], title: &'a str, formatter: fn(&T) -> String) -> Self {
        Self {
            items,
            title,
            selected: 0,
            formatter,
        }
    }

    pub fn selected(mut self, idx: usize) -> Self {
        self.selected = idx;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, highlight_color: Color) {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let content = (self.formatter)(item);
                let style = if i == self.selected {
                    Style::default()
                        .fg(highlight_color)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(content).style(style)
            })
            .collect();

        let list = List::new(items).block(Block::default().title(self.title).borders(Borders::ALL));

        let mut state = ListState::default();
        state.select(Some(self.selected));

        frame.render_stateful_widget(list, area, &mut state);
    }
}
