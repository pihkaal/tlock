use core::panic;
use std::{fs, path::PathBuf};

use crossterm::style::Color;
use ini::configparser::ini::Ini;

use crate::{
    color::{generate_gradient, parse_hex_color, ComputableColor},
    debug,
};

pub struct Config {
    pub be_polite: bool,
    pub fps: u64,
    pub color: ComputableColor,
    pub time_format: String,
    pub date_format: String,
}

const DEFAULT_CONFIG: &str = include_str!("default_config");

pub fn load_from_file(path: PathBuf) -> Config {
    let mut ini = Ini::new();
    ini.load(&path.to_str().unwrap()).unwrap();

    let config = Config {
        be_polite: ini.getbool("general", "polite").unwrap().unwrap(),
        fps: ini.getuint("general", "fps").unwrap().unwrap(),
        color: load_color(&ini),
        time_format: ini.get("format", "time").unwrap(),
        date_format: ini.get("format", "date").unwrap(),
    };

    return config;
}

pub fn write_default_config(path: PathBuf) -> () {
    let parent = path.parent().unwrap();
    let _ = fs::create_dir_all(parent);
    let _ = fs::write(path, DEFAULT_CONFIG);
}

fn load_color(ini: &Ini) -> ComputableColor {
    let color_mode = ini.get("styling", "color_mode").unwrap();

    match color_mode.as_str() {
        "term" => {
            let color = ini.getint("styling", "color_term").unwrap().unwrap();
            return ComputableColor::from(load_term_color(color));
        }
        "hex" => {
            let color = ini.get("styling", "color_hex").unwrap();
            return ComputableColor::from(load_hex_color(&color));
        }
        "ansi" => {
            let color = ini.getint("styling", "color_ansi").unwrap().unwrap();
            return ComputableColor::from(load_ansi_color(color));
        }
        "gradient" => {
            return load_gradient(ini);
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
    let rgb = parse_hex_color(value);
    return Color::Rgb {
        r: rgb.0,
        g: rgb.1,
        b: rgb.2,
    };
}

fn load_ansi_color(value: i64) -> Color {
    return Color::AnsiValue(value.try_into().unwrap());
}

fn load_gradient(ini: &Ini) -> ComputableColor {
    let mut keys = Vec::new();

    let mut i = 0;
    while let Some(key) = ini.get("gradient", &format!("gradient_key_{}", i)) {
        keys.push(parse_hex_color(&key));
        i += 1;
    }

    if !debug::is_debug() && ini.getbool("gradient", "gradient_loop").unwrap().unwrap() {
        let mut loop_keys = keys.clone();
        loop_keys.reverse();
        for i in 1..loop_keys.len() {
            keys.push(*loop_keys.get(i).unwrap());
        }
    }

    let steps: usize = if debug::is_debug() {
        debug::DEBUG_COLOR_DISPLAY_SIZE * 2
    } else {
        ini.getuint("gradient", "gradient_steps")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap()
    };
    return generate_gradient(keys, steps - 1);
}
