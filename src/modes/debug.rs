use std::io::{self, Write};

use clap::crate_version;
use crossterm::{
    queue,
    style::{self, Attribute},
};

use crate::config::Config;

pub const DEBUG_COLOR_DISPLAY_SIZE: usize = 50;

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
    // If width is one, it is a single color
    if width == 1 {
        queue!(stdout, style::SetBackgroundColor(config.color.get_value()))?;
        write!(stdout, "{}", " ".repeat(DEBUG_COLOR_DISPLAY_SIZE))?;
    }
    // Otherwhise, it's a gradient
    else {
        // Use half characters to display two colors in one character using background
        // and foreground
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

    Ok(())
}

fn print_debug_label(key: &str) -> io::Result<()> {
    let mut stdout = io::stdout();

    queue!(stdout, style::SetAttribute(Attribute::Bold))?;
    write!(stdout, "{}: ", key)?;
    queue!(stdout, style::ResetColor)?;

    Ok(())
}
