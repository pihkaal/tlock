use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use chrono;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    queue,
    terminal::{self, ClearType},
};

use crate::{
    config::Config,
    rendering::{self, symbols},
};

pub fn main_loop(config: &mut Config) -> io::Result<()> {
    let mut stdout = io::stdout();

    let mut quit = false;
    while !quit {
        // Handle events
        while event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Key(e) => match e.code {
                    // Handle CTRL-C
                    KeyCode::Char('c') => {
                        if e.modifiers.contains(KeyModifiers::CONTROL) {
                            quit = true;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // Clear frame
        queue!(stdout, terminal::Clear(ClearType::All))?;

        // Render
        render_frame(&config)?;

        config.color.update();

        stdout.flush()?;

        thread::sleep(Duration::from_millis(1000 / config.fps));
    }

    return Ok(());
}

fn render_frame(config: &Config) -> io::Result<()> {
    let color = config.color.get_value();

    let date_time = chrono::Local::now();

    // Display time
    let time = date_time.time().format(&config.time_format).to_string();
    rendering::draw_time(&time, color)?;

    // Display date
    let date = date_time
        .date_naive()
        .format(&config.date_format.to_owned())
        .to_string();

    let (width, height) = rendering::get_terminal_size()?;
    let x = width / 2 - (date.len() as i16) / 2;
    let y = height / 2 + symbols::SYMBOL_HEIGHT as i16 / 2 + 2;
    rendering::draw_text(&date, x, y - 1, color)?;

    return Ok(());
}
