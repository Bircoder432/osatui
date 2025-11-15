pub mod header;
pub mod schedule;
pub mod selector;
pub mod setup;

use crate::app::App;
use ratatui::prelude::*;

pub fn render(f: &mut Frame, app: &App) {
    match app.mode() {
        crate::app::AppMode::Normal => render_normal(f, app),
        crate::app::AppMode::Setup(state) => setup::render_setup(f, app, state),
        crate::app::AppMode::Selector(state) => selector::render_selector(f, app, state),
    }
}

fn render_normal(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    header::render(f, app, chunks[0]);
    schedule::render(f, app, chunks[1]);
}
