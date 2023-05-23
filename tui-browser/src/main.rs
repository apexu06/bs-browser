use std::{fs, io};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use log::LevelFilter;
use simplelog::WriteLogger;
use tui::{backend::CrosstermBackend, Terminal};
use ui::map_browser::start_browser;

mod api;
mod types;
mod ui;
mod utils;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let log_file = fs::File::create("log.txt").unwrap();

    WriteLogger::init(LevelFilter::Info, simplelog::Config::default(), log_file).unwrap();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    start_browser(&mut terminal).await?;

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
