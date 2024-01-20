use std::{
    io::{self, Write},
    sync::atomic::{AtomicBool, Ordering},
};

use clap::crate_version;
use crossterm::{
    queue,
    style::{self, Attribute},
};

use crate::config::Config;

pub const DEBUG_COLOR_DISPLAY_SIZE: usize = 50;

static DEBUG_MODE: AtomicBool = AtomicBool::new(false);

pub fn print_debug_infos(config: &mut Config) -> io::Result<()> {
    let mut stdout = io::stdout();

    print_debug_label("Version")?;
    writeln!(stdout, "{}", crate_version!())?;

    print_debug_label("FPS")?;
    writeln!(stdout, "{}", config.fps)?;

    print_debug_label("Time format")?;
    writeln!(stdout, "{}", config.time_format)?;

    print_debug_label("Date format")?;
    writeln!(stdout, "{}", config.date_format)?;

    print_debug_label("Color scheme")?;
    let width = config.color.get_keys_count();
    if width == 1 {
        queue!(stdout, style::SetBackgroundColor(config.color.get_value()))?;
        write!(stdout, "{}", " ".repeat(DEBUG_COLOR_DISPLAY_SIZE))?;
    } else {
        for _ in 0..width / 2 {
            queue!(stdout, style::SetForegroundColor(config.color.get_value()))?;
            config.color.update();

            queue!(stdout, style::SetBackgroundColor(config.color.get_value()))?;
            config.color.update();

            write!(stdout, "▌")?;
        }
    }

    writeln!(stdout)?;
    queue!(stdout, style::ResetColor)?;
    let _ = stdout.flush();
    return Ok(());
}

fn print_debug_label(key: &str) -> io::Result<()> {
    let mut stdout = io::stdout();

    queue!(stdout, style::SetAttribute(Attribute::Bold))?;
    write!(stdout, "{}: ", key)?;
    queue!(stdout, style::ResetColor)?;

    return Ok(());
}

pub fn enable_debug_mode() -> () {
    DEBUG_MODE.store(true, Ordering::Relaxed);
}

pub fn is_debug() -> bool {
    return DEBUG_MODE.load(Ordering::Relaxed);
}
