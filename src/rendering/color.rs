use crossterm::style::Color;

pub struct ComputableColor {
    values: Vec<Color>,
    current: usize,
}

impl ComputableColor {
    pub fn from(color: Color) -> ComputableColor {
        ComputableColor {
            values: vec![color],
            current: 0,
        }
    }

    pub fn update(&mut self) -> () {
        self.current = (self.current + 1) % self.values.len();
    }

    pub fn get_value(&self) -> Color {
        *self.values.get(self.current).unwrap()
    }

    pub fn get_keys_count(&self) -> usize {
        self.values.len()
    }
}

fn clamp01(v: f32) -> f32 {
    if v < 0.0 {
        0.0
    } else if v > 1.0 {
        1.0
    } else {
        v
    }
}

fn lerp(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) as f32 * clamp01(t)) as u8
}

pub fn generate_gradient(keys: Vec<(u8, u8, u8)>, steps: usize) -> ComputableColor {
    let mut gradient = Vec::with_capacity(steps);

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

    ComputableColor {
        values: gradient,
        current: 0,
    }
}

pub fn parse_hex_color(value: &str) -> (u8, u8, u8) {
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

    (r, g, b)
}
