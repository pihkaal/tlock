use core::panic;

use crossterm::style::Color;
use ini::configparser::ini::Ini;

pub struct Config {
    pub be_polite: bool,
    pub fps: u64,
    pub color: Color,
    pub date_format: String,
}

pub fn load_from_file(path: &str) -> Config {
    let mut ini = Ini::new();
    ini.load(path).unwrap();

    let config = Config {
        be_polite: ini.getbool("general", "polite").unwrap().unwrap(),
        fps: ini.getuint("general", "fps").unwrap().unwrap(),
        color: load_color(&ini),
        date_format: ini.get("format", "date").unwrap(),
    };

    return config;
}

fn load_color(ini: &Ini) -> Color {
    let color_mode = ini.get("styling", "color_mode").unwrap();

    match color_mode.as_str() {
        "term" => {
            let color = ini.getint("styling", "color_term").unwrap().unwrap();
            return load_term_color(color);
        }
        "hex" => {
            let color = ini.get("styling", "color_hex").unwrap();
            return load_hex_color(&color);
        }
        "ansi" => {
            let color = ini.getint("styling", "color_ansi").unwrap().unwrap();
            return load_ansi_color(color);
        }
        _ => panic!("ERROR: Invalid color mode: {}", color_mode),
    }
}

fn load_term_color(value: i64) -> Color {
    match value {
        0 => Color::Black,
        1 => Color::DarkRed,
        2 => Color::DarkGreen,
        3 => Color::DarkYellow,
        4 => Color::DarkBlue,
        5 => Color::DarkMagenta,
        6 => Color::DarkCyan,
        7 => Color::Grey,
        8 => Color::DarkGrey,
        9 => Color::Red,
        10 => Color::Green,
        11 => Color::Yellow,
        12 => Color::Blue,
        13 => Color::Magenta,
        14 => Color::Cyan,
        15 => Color::White,
        _ => panic!("ERROR: Invalid terminal color: {}", value),
    }
}

fn load_hex_color(value: &str) -> Color {
    // Expand #XXX colors
    let value = if value.len() == 3 {
        format!(
            "{}{}{}{}{}{}",
            &value[0..1],
            &value[0..1],
            &value[1..2],
            &value[1..2],
            &value[2..3],
            &value[2..3]
        )
    } else {
        value.to_owned()
    };

    if value.len() != 6 {
        panic!("ERROR: Invalid hex color: {}", value);
    }

    let r = u8::from_str_radix(&value[0..2], 16).unwrap();
    let g = u8::from_str_radix(&value[2..4], 16).unwrap();
    let b = u8::from_str_radix(&value[4..6], 16).unwrap();

    return Color::Rgb { r, g, b };
}

fn load_ansi_color(value: i64) -> Color {
    return Color::AnsiValue(value.try_into().unwrap());
}
