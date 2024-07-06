use std::io::{self};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use crossterm::event::{self, KeyCode};
use crossterm::{
    ExecutableCommand, 
    terminal,
};

mod util;
use crossterm::event::Event;
// use util::config::Config;
use util::screen::Screen;

fn main() -> io::Result<()> {
    let screen = Arc::new(Mutex::new(Screen::new(50, 20)));

    //display thread
    let display = Arc::clone(&screen);
    let _ = thread::spawn(move || {
        loop {
            {
                let mut state = display.lock().unwrap();
                state.update();
            }
            thread::sleep(time::Duration::from_millis(100));
        }
    });

    //raw mode for handling keys
    let _ = terminal::enable_raw_mode();

    loop {
        if event::poll(std::time::Duration::from_millis(10))? {
            // Read the event
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(c) => {
                        let mut state = screen.lock().unwrap();
                        if c == ' ' {
                            state.mvt();
                        }
                        if c == 'r' {
                            state.reset();
                        }
                        if c == 'q' {
                            let mut stdout = io::stdout();
                            stdout.execute(terminal::Clear(terminal::ClearType::All))?;

                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let _ = terminal::disable_raw_mode();
    Ok(())
}
