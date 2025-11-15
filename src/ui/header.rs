use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let date_str = app.date().format();

    let header = Paragraph::new(format!(" {date_str} | F1 ← | F2 Сегодня | F3 → | q Выход "))
        .style(
            Style::default()
                .bg(theme.color("header_bg"))
                .fg(theme.color("header_fg")),
        )
        .alignment(Alignment::Center);

    f.render_widget(header, area);
}
