use core::panic;
use std::{
    io::{self, Write},
    path::PathBuf,
    thread,
    time::Duration,
};

use chrono::Local;
use clap::{Parser, Subcommand};
use config::{write_default_config, Config};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute, queue,
    style::{self, Attribute, Color},
    terminal::{self, ClearType},
};
use debug::print_debug_infos;
use dirs::config_dir;

mod color;
mod config;
mod debug;
mod symbols;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,

    #[arg(short, long, action)]
    regenerate_default: bool,

    #[arg(short, long, action)]
    yes: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Debug {},
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Debug {}) => {
            debug::enable_debug_mode();
        }
        _ => {}
    }

    // Load config
    let mut default_generated = false;
    let config_file = if let Some(custom_config) = cli.config {
        PathBuf::from(custom_config)
    } else {
        let config_file = config_dir().unwrap().join("tlock").join("config");
        if !config_file.exists() {
            write_default_config(config_file.clone());
            default_generated = true;
        }

        config_file
    };

    if cli.regenerate_default {
        if !default_generated && config_file.exists() && !cli.yes {
            println!("A config file is already located at {:?}", config_file);
            print!("Do you really want to recreate it ? [y/N] ");

            let _ = io::stdout().flush();

            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input);

            let response = input.trim().to_lowercase();
            if response != "y" {
                println!("Cancelled.");
                return Ok(());
            }
        }

        write_default_config(config_file.clone());
        println!("Done.");
        return Ok(());
    }

    if !config_file.exists() {
        panic!("ERROR: Configuration file not found");
    }

    let mut config = config::load_from_file(config_file);
    let mut stdout = io::stdout();

    match &cli.command {
        Some(Commands::Debug {}) => {
            print_debug_infos(&mut config)?;
            return Ok(());
        }
        _ => {}
    }

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
