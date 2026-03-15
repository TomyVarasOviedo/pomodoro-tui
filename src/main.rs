mod config;
mod timer;
mod ui;
mod notify;

use std::{io, time::Duration};

use config::Config;
use crossterm::{event::{self, Event, KeyCode, KeyModifiers}, 
                execute, terminal::{EnterAlternateScreen, 
                LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};
use ratatui::{Terminal, backend::CrosstermBackend};
use timer::Timer;

fn main() -> io::Result<()>{
    let config = Config::load();
    let mut timer = Timer::new(config.clone());
    
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let tick = Duration::from_millis(500);

    loop {
        terminal.draw(|f| ui::render(f, &timer))?;

        if event::poll(tick)? {
            if let Event::Key(key) = event::read()? {
                if key.modifiers.contains(KeyModifiers::CONTROL) 
                && matches!(key.code, KeyCode::Char('c') | KeyCode::Char('d')) {
                    break;
                    
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => break,

                    KeyCode::Char(' ') => {
                        timer.toggle();
                    }

                    KeyCode::Char('s') | KeyCode::Char('S') =>{
                        timer.skip();
                        notify::send_notification(&timer, &config);
                    }

                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        timer.reset();
                    }

                    _ => {}
                }                
            }
        }

        if timer.tick() {
            notify::send_notification(&timer, &config);
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
