use std::{any::type_name, fs, path::PathBuf};

use crossterm::style::Color;
use ini::configparser::ini::Ini;

use crate::{
    eprintln_quit,
    modes::debug,
    rendering::color::{generate_gradient, parse_hex_color, ComputableColor},
};

pub struct Config {
    pub be_polite: bool,
    pub fps: u64,
    pub color: ComputableColor,
    pub time_format: String,
    pub date_format: String,
}

const DEFAULT_CONFIG: &str = include_str!("default_config");

pub fn load_from_file(path: PathBuf, debug_mode: bool) -> Config {
    let mut ini = Ini::new();
    ini.load(
        &path
            .to_str()
            .unwrap_or_else(|| eprintln_quit!("Invalid configuration path")),
    )
    .unwrap_or_else(|_| eprintln_quit!("Unable to parse configuration file"));

    Config {
        be_polite: get_ini_value(&ini, "general", "polite"),
        fps: get_ini_value(&ini, "general", "fps"),
        color: load_color(&ini, debug_mode),
        time_format: get_ini_value(&ini, "format", "time"),
        date_format: get_ini_value(&ini, "format", "date"),
    }
}

pub fn write_default_config(path: PathBuf) -> () {
    // Write default config file to target path
    let parent = path
        .parent()
        .unwrap_or_else(|| eprintln_quit!("Invalid configuration path"));
    let _ = fs::create_dir_all(parent);
    let _ = fs::write(path, DEFAULT_CONFIG);
}

fn get_ini_value<T: std::str::FromStr>(ini: &Ini, section: &str, key: &str) -> T {
    if let Some(value) = ini.get(section, key) {
        value.parse::<T>().unwrap_or_else(|_| {
            eprintln_quit!(
                "Invalid value at {}.{}: Expected {}, got '{}'",
                section,
                key,
                type_name::<T>(),
                value
            )
        })
    } else {
        eprintln_quit!("Missing required config key: {}.{}", section, key)
    }
}

fn load_color(ini: &Ini, debug_mode: bool) -> ComputableColor {
    let color_mode: String = get_ini_value(&ini, "styling", "color_mode");

    match color_mode.as_str() {
        "term" => {
            let color: u8 = get_ini_value(&ini, "styling", "color_term");
            ComputableColor::from(load_term_color(color))
        }
        "hex" => {
            let color: String = get_ini_value(&ini, "styling", "color_hex");
            ComputableColor::from(load_hex_color(&color))
        }
        "ansi" => {
            let color: u8 = get_ini_value(&ini, "styling", "color_ansi");
            ComputableColor::from(load_ansi_color(color))
        }
        "gradient" => load_gradient(ini, debug_mode),
        _ => eprintln_quit!("Invalid color mode: {}", color_mode),
    }
}

fn load_term_color(value: u8) -> Color {
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
        _ => eprintln_quit!("Invalid terminal color: {}", value),
    }
}

fn load_hex_color(value: &str) -> Color {
    let rgb = parse_hex_color(value);
    Color::Rgb {
        r: rgb.0,
        g: rgb.1,
        b: rgb.2,
    }
}

fn load_ansi_color(value: u8) -> Color {
    Color::AnsiValue(value)
}

fn load_gradient(ini: &Ini, debug_mode: bool) -> ComputableColor {
    let mut keys = Vec::new();

    // Iterate over all gradient keys, they are defined like that in the config file:
    //   gradient_key_1=...
    //   gradient_key_2=...
    //   gradient_key_N=...
    let mut i = 0;
    while let Some(key) = ini.get("gradient", &format!("gradient_key_{}", i)) {
        keys.push(parse_hex_color(&key));
        i += 1;
    }

    // Generate gradient loop if needed
    if !debug_mode && get_ini_value(&ini, "gradient", "gradient_loop") {
        for &key in keys.clone().iter().rev().skip(1) {
            keys.push(key);
        }
    }

    // I use half characters for debug mode rendering, so we take display size * 2
    let steps: usize = if debug_mode {
        debug::DEBUG_COLOR_DISPLAY_SIZE * 2
    } else {
        get_ini_value(&ini, "gradient", "gradient_steps")
    };
    generate_gradient(keys, steps - 1)
}
