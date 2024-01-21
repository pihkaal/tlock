use std::io::{self, Write};

use crossterm::{
    cursor, queue,
    style::{self, Attribute, Color},
    terminal,
};

pub mod color;
pub mod symbols;

pub fn draw_time(time: &str, color: Color) -> io::Result<()> {
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

pub fn draw_text(string: &str, x: u16, y: u16, color: Color) -> io::Result<()> {
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
