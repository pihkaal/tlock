use core::panic;

use crossterm::style::Color;
use ini::configparser::ini::Ini;

pub struct Config {
    pub be_polite: bool,
    pub fps: u64,
    pub color: Color,
}

pub fn load_from_file(path: &str) -> Config {
    let mut ini = Ini::new();
    ini.load(path).unwrap();

    let config = Config {
        be_polite: ini.getbool("general", "polite").unwrap().unwrap(),
        fps: ini.getuint("general", "fps").unwrap().unwrap(),
        color: load_color(&ini),
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
        "rgb" => todo!(),
        "ansi" => todo!(),
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
