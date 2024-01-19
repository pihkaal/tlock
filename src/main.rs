use core::panic;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    thread,
    time::Duration,
};

use chrono::Local;
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

    let mut config = config::load_from_file(&config_file.to_str().unwrap());

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

        config.color.update();

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

    let date_time = Local::now();

    // Display time
    let time = date_time.time().format(&config.time_format).to_string();

    let text_width = draw_time_width(&time);
    let text_height = 5;
    let color = config.color.get_value();

    let x = width / 2 - text_width / 2;
    let y = height / 2 - text_height / 2;
    draw_time(&time, x, y, color)?;

    // Display date
    let date = date_time
        .date_naive()
        .format(&config.date_format.to_owned())
        .to_string();

    let x = width / 2 - (date.len() as u16) / 2;
    let y = height / 2 + text_height / 2 + 2;
    draw_date(&date, x, y, color)?;

    return Ok(());
}

fn draw_time_width(time: &str) -> u16 {
    if time.len() == 0 {
        return 0;
    }

    let mut w = 0;
    for c in time.chars() {
        w += if c == ':' { 6 } else { 7 };
    }

    w -= if time.len() == 1 { 1 } else { 2 };
    return w;
}

fn draw_time(time: &str, mut x: u16, y: u16, color: Color) -> io::Result<()> {
    for c in time.chars() {
        if c == ':' {
            x -= 1;
        }

        draw_time_symbol(c, x, y, color)?;
        x += 7;

        if c == ':' {
            x -= 1;
        }
    }

    return Ok(());
}

fn draw_time_symbol(symbol: char, x: u16, y: u16, color: Color) -> io::Result<()> {
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

fn draw_date(date: &str, x: u16, y: u16, color: Color) -> io::Result<()> {
    let mut stdout = io::stdout();

    queue!(
        stdout,
        cursor::MoveTo(x, y),
        style::SetForegroundColor(color),
        style::SetAttribute(Attribute::Bold)
    )?;
    write!(stdout, "{}", date)?;

    return Ok(());
}
