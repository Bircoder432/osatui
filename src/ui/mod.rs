pub mod header;
pub mod schedule;

use crate::app::App;
use ratatui::prelude::*;

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    header::render(f, app, chunks[0]);
    schedule::render(f, app, chunks[1]);
}
