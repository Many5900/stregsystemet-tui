use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use std::io::{self, Write};

mod api;
mod app;
mod config;
mod error;
mod models;
mod ui;
mod utils;

use crate::ui::events::EventHandler;
use app::state::AppState;
use config::store::load_or_create_config;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    let config_result = load_or_create_config();
    let config = config_result.unwrap_or_default();

    let app_state = AppState::new(config);

    let shutdown_flag = Arc::new(Mutex::new(false));

    let mut event_handler = EventHandler::new(&mut terminal, app_state, shutdown_flag.clone());
    let result = event_handler.run().await;

    cleanup_terminal(&mut terminal)?;

    if let Err(err) = result {
        let _ = cleanup_terminal(&mut terminal);

        eprintln!("Error: {err:?}");
        return Err(io::Error::other(err.to_string()));
    }

    Ok(())
}

fn cleanup_terminal(
    terminal: &mut ratatui::Terminal<CrosstermBackend<io::Stdout>>,
) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    io::stdout().flush()?;
    std::thread::sleep(std::time::Duration::from_millis(50));

    Ok(())
}
