mod app;
mod ui;

use anyhow::Result;
use app::App;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{prelude::*, Terminal};
use std::io;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => {
                    if app.mode == app::AppMode::List && app.main_tab == app::MainTab::Servers {
                        app.next_server();
                    }
                }
                KeyCode::Up => {
                    if app.mode == app::AppMode::List && app.main_tab == app::MainTab::Servers {
                        app.previous_server();
                    }
                }
                KeyCode::Enter => {
                    app.enter_detail();
                }
                KeyCode::Esc => {
                    if app.mode == app::AppMode::Detail {
                        app.back_to_list();
                    }
                }
                KeyCode::Right => {
                    app.next_menu();
                }
                KeyCode::Left => {
                    app.previous_menu();
                }
                _ => {}
            }
        }
    }
}

