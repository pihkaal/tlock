use core::panic;
use std::{
    cmp::min,
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

struct Lapse {
    pub time: time::Duration,
    pub delta: time::Duration,
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

    // For the chronometer
    let start_time = time::Instant::now();
    let mut lapses: Vec<Lapse> = vec![];

    // Main loop
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
                    // Handle lapse
                    KeyCode::Char(' ') => {
                        if get_app_mode() == AppMode::Chrono {
                            let time = start_time.elapsed();
                            let delta = if let Some(last_lap) = lapses.last() {
                                time::Duration::from_secs(time.as_secs() - last_lap.time.as_secs())
                            } else {
                                time
                            };

                            lapses.push(Lapse { time, delta });
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
            AppMode::Chrono => render_chrono(&config, start_time, &lapses)?,
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

    let (width, height) = terminal::size()?;
    let x = width / 2 - (date.len() as u16) / 2;
    let y = height / 2 + symbols::SYMBOL_HEIGHT as u16 / 2 + 2;
    draw_text(&date, x, y, color)?;

    return Ok(());
}

fn render_chrono(
    config: &Config,
    start_time: time::Instant,
    lapses: &Vec<Lapse>,
) -> io::Result<()> {
    let color = config.color.get_value();

    // Display time
    let elapsed = format_duration(start_time.elapsed());
    draw_time(&elapsed, color)?;

    // Display lapses
    let (width, height) = terminal::size()?;
    let y = height / 2 + symbols::SYMBOL_HEIGHT as u16 / 2 + 2;
    let max_items = min(10, height - y) as usize;

    for (i, lapse) in lapses.iter().rev().take(max_items).enumerate() {
        let delta = format_duration(lapse.delta);
        let time = format_duration(lapse.time);

        let lapse = format!("#0{}  --  +{}  --  {}", lapses.len() - i, delta, time);
        let x = width / 2 - (lapse.len() as u16) / 2;
        draw_text(&lapse, x, y + i as u16, color)?;
    }

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

fn draw_text(string: &str, x: u16, y: u16, color: Color) -> io::Result<()> {
    let mut stdout = io::stdout();

    queue!(
        stdout,
        cursor::MoveTo(x, y),
        style::SetForegroundColor(color),
        style::SetAttribute(Attribute::Bold)
    )?;
    write!(stdout, "{}", string)?;

    return Ok(());
}

fn format_duration(duration: time::Duration) -> String {
    let seconds = duration.as_secs();
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    return format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
}
