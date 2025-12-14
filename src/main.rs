mod api;
mod app;
mod cache;
mod config;
mod ui;
mod utils;

use app::App;
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::Config::load().await?;
    let mut app = App::new(config).await?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match app.mode() {
                        app::AppMode::Normal => match key.code {
                            KeyCode::Char('q') => app.quit(),
                            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                app.start_setup();
                            }
                            KeyCode::Char('o') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                if let Err(e) = app.start_selector().await {
                                    eprintln!("Ошибка запуска селектора: {}", e);
                                }
                            }
                            KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::SHIFT) => {
                                if let Err(e) = app.reload_cache().await {
                                    eprintln!("Ошибка перезагрузки кеша: {}", e);
                                }
                            }
                            KeyCode::F(1) => {
                                if let Err(e) = app.prev_day().await {
                                    eprintln!("Ошибка перехода к предыдущему дню: {}", e);
                                }
                            }
                            KeyCode::F(2) => {
                                if let Err(e) = app.go_today().await {
                                    eprintln!("Ошибка перехода к сегодняшнему дню: {}", e);
                                }
                            }
                            KeyCode::F(3) => {
                                if let Err(e) = app.next_day().await {
                                    eprintln!("Ошибка перехода к следующему дню: {}", e);
                                }
                            }
                            _ => {}
                        },
                        app::AppMode::Setup(_) => {}
                        app::AppMode::Selector(_) => match key.code {
                            KeyCode::Enter => {
                                if let Err(e) = app.handle_selector_input().await {
                                    eprintln!("Ошибка обработки выбора: {}", e);
                                }
                            }
                            KeyCode::Down | KeyCode::Up | KeyCode::Right | KeyCode::Left => {
                                app.handle_selector_navigation(key.code).await;
                            }
                            KeyCode::Esc => {
                                *app.mode_mut() = app::AppMode::Normal;
                            }
                            _ => {}
                        },
                    }
                }
            }
        }

        if app.should_quit() {
            break;
        }
    }

    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    Ok(())
}
