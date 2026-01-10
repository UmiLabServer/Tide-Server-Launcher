mod app;
mod events;
mod ui;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
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
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }
    
    Ok(())
}
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        // 最初に現在の状態を描画
        terminal.draw(|f| ui::ui(f, app))?;
        
        // debug
        std::fs::write("debug.log", format!("locate:[{}, {}]\nitem: [{}, {}]\ndepth: {}", app.locate[0], app.locate[1], app.item[0], app.item[1], app.depth))?;
        
        // キー入力を処理
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => {
                    if !app.items.is_empty() && app.locate[app.depth] == 0 {
                        app.item[app.depth] = (app.item[app.depth] + 1) % app.items.len();
                    }
                }
                KeyCode::Up => {
                    if !app.items.is_empty() && app.locate[app.depth] == 0 {
                        if app.item[app.depth] == 0 {
                            app.item[app.depth] = app.items.len() - 1;
                        } else {
                            app.item[app.depth] -= 1;
                        }
                    }
                }
                KeyCode::Enter => {
                    if app.locate[app.depth] == 0 && !app.items.is_empty() && app.locate[app.depth] == 0 {
                        app.selected_server_name = app.items[app.item[app.depth]].name.to_string();
                        let _ = app.save_config();
                        app.depth = 1;
                        app.locate[app.depth] = 0;
                    }
                }
                KeyCode::Esc => {
                    if app.depth == 1 {
                        app.depth = 0;
                    }
                }
                KeyCode::Right => {
                    app.locate[app.depth] = (app.locate[app.depth] + 1) % app.menu.len();
                    app.item[app.depth] = 0;
                }
                KeyCode::Left => {
                    if app.locate[app.depth] == 0 {
                        app.locate[app.depth] = app.menu.len() - 1;
                    } else {
                        app.locate[app.depth] -= 1;
                    }
                    app.item[app.depth] = 0;
                }
                _ => {}
            }
        }
    }
}
