use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use chrono::{Local, Timelike};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute, queue,
    style::{self, Color},
    terminal::{self, ClearType},
};

mod symbols;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    // Switch to alternate screen, hide the cursor and enable raw mode
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
    let _ = terminal::enable_raw_mode()?;

    // Main loop
    let mut quit = false;
    while !quit {
        // Handle events
        while event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Key(e) => match e.code {
                    KeyCode::Char(x) => {
                        // Handle CTRL-C
                        if x == 'c' && e.modifiers.contains(KeyModifiers::CONTROL) {
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
        render_frame()?;

        stdout.flush()?;

        // 30fps
        thread::sleep(Duration::from_millis(33));
    }

    // Disale raw mode, leave the alternate screen and show the cursor back
    let _ = terminal::disable_raw_mode().unwrap();
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;

    // Be polite
    println!("CTRL-C pressed, bye!\n");

    return Ok(());
}

fn render_frame() -> io::Result<()> {
    let (width, height) = terminal::size()?;

    let time = Local::now();
    let hour = time.hour().to_string();
    let minute = time.minute().to_string();

    // Display current time
    let text_width = 6 + 7 + 6 + 7 + 6;
    let text_height = 5;
    let color = Color::White;

    let x = width / 2 - text_width / 2;
    let y = height / 2 - text_height / 2;

    // Hour
    if hour.len() == 1 {
        draw_symbol('0', x - 0 + 0 * 7, y, color)?;
    } else {
        draw_symbol(hour.chars().nth(0).unwrap(), x - 0 + 0 * 7, y, color)?;
    }
    draw_symbol(hour.chars().last().unwrap(), x - 0 + 1 * 7, y, color)?;

    draw_symbol(':', x - 1 + 2 * 7, y, color)?;

    // Minutes
    if minute.len() == 1 {
        draw_symbol('0', x - 2 + 3 * 7, y, color)?;
    } else {
        draw_symbol(minute.chars().nth(0).unwrap(), x - 2 + 3 * 7, y, color)?;
    }
    draw_symbol(minute.chars().last().unwrap(), x - 2 + 4 * 7, y, color)?;

    return Ok(());
}

fn draw_symbol(symbol: char, x: u16, y: u16, color: Color) -> io::Result<()> {
    let mut stdout = io::stdout();

    let data = symbols::symbol_to_render_data(symbol);

    for oy in 0..data.len() {
        for ox in 0..data[oy].len() {
            if data[oy][ox] {
                let cx = ox as u16;
                let cy = oy as u16;

                // Render cursor at position by setting background color and using space
                queue!(
                    stdout,
                    cursor::MoveTo(x + cx, y + cy),
                    style::SetBackgroundColor(color)
                )?;
                write!(stdout, " ")?;
                queue!(stdout, style::ResetColor)?;
            }
        }
    }

    return Ok(());
}
