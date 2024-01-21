use core::panic;
use std::{
    io::{self, Write},
    path::PathBuf,
    sync::atomic::Ordering,
};

use atomic_enum::atomic_enum;
use clap::{Parser, Subcommand};
use config::write_default_config;
use crossterm::{cursor, execute, terminal};
use dirs::config_dir;

use crate::modes::debug;

mod config;
mod modes;
mod rendering;
mod utils;

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
            debug::print_debug_infos(&mut config)?;
            return Ok(());
        }
        _ => {}
    }

    // Switch to alternate screen, hide the cursor and enable raw mode
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
    let _ = terminal::enable_raw_mode()?;

    match get_app_mode() {
        AppMode::Clock => modes::clock::main_loop(&mut config)?,
        AppMode::Chrono => modes::chrono::main_loop(&mut config)?,
        AppMode::Debug => unreachable!(),
    };

    // Disale raw mode, leave the alternate screen and show the cursor back
    let _ = terminal::disable_raw_mode().unwrap();
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;

    // Be polite
    if config.be_polite {
        println!("CTRL-C pressed, bye!\n");
    }

    return Ok(());
}
