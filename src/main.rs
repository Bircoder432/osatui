use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use log::info;
use osatui::{app::App, config::Config};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    info!("Starting osatui v{}", env!("CARGO_PKG_VERSION"));

    let config = Config::load().await?;
    info!("Configuration loaded successfully");

    let mut app = App::new(config).await?;
    info!("Application initialized");

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;

    result
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> anyhow::Result<()> {
    let mut last_tick = tokio::time::Instant::now();
    let tick_rate = tokio::time::Duration::from_millis(250);

    loop {
        terminal.draw(|f| osatui::ui::render(f, app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        if crossterm::event::poll(timeout)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && let Err(e) = app.handle_key_event(key).await
        {
            log::error!("Error handling key event: {}", e);
            app.set_error_message(format!("Error: {}", e));
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = tokio::time::Instant::now();
        }

        if app.should_quit() {
            break;
        }
    }

    Ok(())
}
