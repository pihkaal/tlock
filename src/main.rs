use core::panic;
use std::{
    io::{self, Write},
    path::PathBuf,
    sync::atomic::Ordering,
    thread,
    time::{self, Duration},
};

use atomic_enum::atomic_enum;
use chrono;
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
    Chrono {},
}

#[atomic_enum]
#[derive(PartialEq)]
pub enum AppMode {
    Clock = 0,
    Debug,
    Chrono,
}

static APP_MODE: AtomicAppMode = AtomicAppMode::new(AppMode::Debug);

pub fn get_app_mode() -> AppMode {
    return APP_MODE.load(Ordering::Relaxed);
}

pub fn set_app_mode(mode: AppMode) {
    return APP_MODE.store(mode, Ordering::Relaxed);
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Debug {}) => {
            set_app_mode(AppMode::Debug);
        }
        Some(Commands::Chrono {}) => set_app_mode(AppMode::Chrono),
        _ => set_app_mode(AppMode::Clock),
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

    match get_app_mode() {
        AppMode::Debug => {
            print_debug_infos(&mut config)?;
            return Ok(());
        }
        _ => {}
    }

    // Switch to alternate screen, hide the cursor and enable raw mode
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
    let _ = terminal::enable_raw_mode()?;

    let start_time = time::Instant::now();

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
        match get_app_mode() {
            AppMode::Clock => render_clock(&config)?,
            AppMode::Chrono => render_chrono(&config, &start_time)?,
            AppMode::Debug => unreachable!(),
        };

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

fn render_clock(config: &Config) -> io::Result<()> {
    let color = config.color.get_value();

    let date_time = chrono::Local::now();

    // Display time
    let time = date_time.time().format(&config.time_format).to_string();
    draw_time(&time, color)?;

    // Display date
    let date = date_time
        .date_naive()
        .format(&config.date_format.to_owned())
        .to_string();
    draw_date(&date, color)?;

    return Ok(());
}

fn render_chrono(config: &Config, start_time: &time::Instant) -> io::Result<()> {
    let color = config.color.get_value();

    // Display time
    let seconds = start_time.elapsed().as_secs();
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    let elapsed = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
    draw_time(&elapsed, color)?;

    return Ok(());
}

fn draw_time_width(time: &str) -> u16 {
    if time.len() == 0 {
        return 0;
    }

    let mut w = 0;
    for c in time.chars() {
        w += if c == ':' {
            symbols::SYMBOL_HEIGHT
        } else {
            symbols::SYMBOL_WIDTH + 1
        };
    }

    w -= if time.len() == 1 { 1 } else { 2 };
    return w.try_into().unwrap();
}

fn draw_time(time: &str, color: Color) -> io::Result<()> {
    let (width, height) = terminal::size()?;

    let text_width = draw_time_width(&time);
    let text_height = 5;

    let mut x = width / 2 - text_width / 2;
    let y = height / 2 - text_height / 2;
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

fn draw_date(date: &str, color: Color) -> io::Result<()> {
    let mut stdout = io::stdout();

    let (width, height) = terminal::size()?;

    let x = width / 2 - (date.len() as u16) / 2;
    let y = height / 2 + symbols::SYMBOL_HEIGHT as u16 / 2 + 2;

    queue!(
        stdout,
        cursor::MoveTo(x, y),
        style::SetForegroundColor(color),
        style::SetAttribute(Attribute::Bold)
    )?;
    write!(stdout, "{}", date)?;

    return Ok(());
}
