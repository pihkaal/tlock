use core::panic;
use std::{fs, path::PathBuf};

use crossterm::style::Color;
use ini::configparser::ini::Ini;

pub struct ComputableColor {
    values: Vec<Color>,
    current: usize,
}

impl ComputableColor {
    fn from(color: Color) -> ComputableColor {
        return ComputableColor {
            values: vec![color],
            current: 0,
        };
    }

    pub fn update(&mut self) {
        self.current = (self.current + 1) % self.values.len();
    }

    pub fn get_value(&self) -> Color {
        return *self.values.get(self.current).unwrap();
    }
}

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

fn parse_hex_color(value: &str) -> (u8, u8, u8) {
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

    return (r, g, b);
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

fn clamp01(v: f32) -> f32 {
    return if v < 0.0 {
        0.0
    } else if v > 1.0 {
        1.0
    } else {
        v
    };
}

fn lerp(a: u8, b: u8, t: f32) -> u8 {
    let v = a as f32 + (b as f32 - a as f32) as f32 * clamp01(t);
    return v as u8;
}

fn load_gradient(ini: &Ini) -> ComputableColor {
    let mut keys = Vec::new();

    let mut i = 0;
    while let Some(key) = ini.get("gradient", &format!("gradient_key_{}", i)) {
        keys.push(parse_hex_color(&key));
        i += 1;
    }

    if ini.getbool("gradient", "gradient_loop").unwrap().unwrap() {
        let mut loop_keys = keys.clone();
        loop_keys.reverse();
        for i in 1..loop_keys.len() {
            keys.push(*loop_keys.get(i).unwrap());
        }
    }

    let steps = ini.getuint("gradient", "gradient_steps").unwrap().unwrap();
    let mut gradient = Vec::with_capacity(steps.try_into().unwrap());

    let step_size = 1.0 / (steps as f32 / (keys.len() as f32 - 1.0));
    for i in 0..keys.len() - 1 {
        let current = keys.get(i).unwrap();
        let next = keys.get(i + 1).unwrap();

        let mut t = 0.0;
        while t <= 1.0 {
            t += step_size;

            let r = lerp(current.0, next.0, t);
            let g = lerp(current.1, next.1, t);
            let b = lerp(current.2, next.2, t);

            gradient.push(Color::Rgb { r, g, b });
        }
    }

    return ComputableColor {
        values: gradient,
        current: 0,
    };
}
