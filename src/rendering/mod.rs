use std::{
    cmp::min,
    i16,
    io::{self, Write},
};

use crossterm::{
    cursor, queue,
    style::{self, Attribute, Color},
    terminal,
};

pub mod color;
pub mod symbols;

pub fn get_terminal_size() -> io::Result<(i16, i16)> {
    let (width, height) = terminal::size()?;

    Ok((width as i16, height as i16))
}

pub fn draw_time(time: &str, color: Color) -> io::Result<()> {
    let (width, height) = get_terminal_size()?;

    let text_width = draw_time_width(&time);
    let text_height = 5;

    let mut x = width / 2 - text_width / 2 - 1;
    let y = height / 2 - text_height / 2 - 1;
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

pub fn draw_text(mut string: &str, mut x: i16, y: i16, color: Color) -> io::Result<()> {
    let mut stdout = io::stdout();
    let (width, _) = get_terminal_size()?;

    let offset = if x < 0 {
        let ret = -x as usize;
        x = 0;
        ret
    } else {
        0
    };
    string = &string[offset..min(string.len(), offset + width as usize)];

    queue!(
        stdout,
        cursor::MoveTo(x as u16, y as u16),
        style::SetForegroundColor(color),
        style::SetAttribute(Attribute::Bold)
    )?;
    write!(stdout, "{}", string)?;
    return Ok(());
}

fn draw_time_width(time: &str) -> i16 {
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

    w.try_into().unwrap()
}

fn draw_time_symbol(symbol: char, x: i16, y: i16, color: Color) -> io::Result<()> {
    let mut stdout = io::stdout();
    let (width, height) = get_terminal_size()?;

    let data = symbols::symbol_to_render_data(symbol);

    for oy in 0..data.len() {
        for ox in 0..data[oy].len() {
            if data[oy][ox] {
                let cx = x + ox as i16;
                let cy = y + oy as i16;

                if cx < 0 || cx >= width || cy < 0 || cy >= height {
                    continue;
                }

                // Render cursor at position by setting background color and using space
                queue!(
                    stdout,
                    cursor::MoveTo(cx as u16, cy as u16),
                    style::SetBackgroundColor(color)
                )?;
                write!(stdout, " ")?;
                queue!(stdout, style::ResetColor)?;
            }
        }
    }

    return Ok(());
}
