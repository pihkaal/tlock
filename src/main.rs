use core::panic;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    thread,
    time::Duration,
};

use chrono::{Local, Timelike};
use clap::Parser;
use config::Config;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute, queue,
    style::{self, Attribute, Color},
    terminal::{self, ClearType},
};
use dirs::config_dir;

mod config;
mod symbols;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,
}

const DEFAULT_CONFIG: &str = include_str!("default_config");

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Load config
    let config_file = if let Some(custom_config) = args.config {
        PathBuf::from(custom_config)
    } else {
        let config_dir = config_dir().unwrap().join("tlock");
        let config_file = config_dir.clone().join("config");
        if !config_file.exists() {
            // Generate default config
            let _ = fs::create_dir(config_dir);
            let _ = fs::write(config_file.clone(), DEFAULT_CONFIG);
        }

        config_file
    };
    if !config_file.exists() {
        panic!("ERROR: Configuration file not found");
    }

    let config = config::load_from_file(&config_file.to_str().unwrap());

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
        render_frame(&config)?;

        stdout.flush()?;

        thread::sleep(Duration::from_millis(1000 / config.fps));
    }

    // Disale raw mode, leave the alternate screen and show the cursor back
    let _ = terminal::disable_raw_mode().unwrap();
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;

    // Be polite
    if config.be_polite {
        println!("CTRL-C pressed, bye!\n");
    }

    return Ok(());
}

fn render_frame(config: &Config) -> io::Result<()> {
    let (width, height) = terminal::size()?;

    let time = Local::now();
    let hour = time.hour().to_string();
    let minute = time.minute().to_string();

    // Display current time
    let text_width = 6 + 7 + 6 + 7 + 6;
    let text_height = 5;
    let color = config.color;

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

    // Display date
    let date = time
        .date_naive()
        .format(&config.date_format.to_owned())
        .to_string();
    let mut stdout = io::stdout();

    let x = width / 2 - (date.len() as u16) / 2;
    let y = height / 2 + text_height / 2 + 2;

    queue!(
        stdout,
        cursor::MoveTo(x, y),
        style::SetForegroundColor(color),
        style::SetAttribute(Attribute::Bold)
    )?;
    write!(stdout, "{}", date)?;

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
