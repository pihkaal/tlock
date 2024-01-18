use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute, queue,
    style::{self, Color},
    terminal::{self, ClearType},
};

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
    let mut stdout = io::stdout();
    let (width, height) = terminal::size()?;

    // Render red X at middle of screen
    queue!(
        stdout,
        cursor::MoveTo(width / 2, height / 2),
        style::SetForegroundColor(Color::Red)
    )?;

    write!(stdout, "X")?;

    return Ok(());
}
