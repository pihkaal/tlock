use std::{
    cmp::min,
    io::{self, Write},
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    queue,
    terminal::{self, ClearType},
};

use crate::utils;
use crate::{
    config::Config,
    rendering::{self, symbols},
};

struct Lapse {
    pub time: Duration,
    pub delta: Duration,
}

struct Chronometer {
    start_time: Option<Instant>,
    paused_duration: Duration,
}

impl Chronometer {
    fn new() -> Self {
        Chronometer {
            start_time: None,
            paused_duration: Duration::from_secs(0),
        }
    }

    fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    fn reset(&mut self) {
        self.start_time = None;
        self.paused_duration = Duration::from_secs(0);
    }

    fn toggle_pause(&mut self) {
        if let Some(start_time) = self.start_time {
            self.paused_duration += Instant::now().duration_since(start_time);
            self.start_time = None;
        } else {
            self.start_time = Some(Instant::now());
        }
    }

    fn is_paused(&self) -> bool {
        self.start_time.is_none()
    }

    fn elapsed(&self) -> Duration {
        if let Some(start_time) = self.start_time {
            Instant::now().duration_since(start_time) + self.paused_duration
        } else {
            self.paused_duration
        }
    }
}

pub fn main_loop(config: &mut Config) -> io::Result<()> {
    let mut stdout = io::stdout();

    let mut chronometer = Chronometer::new();
    chronometer.start();

    let mut lapses: Vec<Lapse> = vec![];
    let mut scroll_offset: usize = 0;

    let mut quit = false;
    while !quit {
        // Handle events
        while event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Key(e) => match e.code {
                    // Handle CTRL-C
                    KeyCode::Char('c') => {
                        if e.modifiers.contains(KeyModifiers::CONTROL) {
                            quit = true;
                        }
                    }
                    // Handle pause
                    KeyCode::Char(' ') => {
                        chronometer.toggle_pause();
                    }
                    // Handle reset
                    KeyCode::Char('r') => {
                        chronometer.reset();
                        lapses.clear();
                        scroll_offset = 0;
                    }
                    // Handle lapses
                    KeyCode::Char('l') => {
                        let time = chronometer.elapsed();
                        let delta = if let Some(last_lap) = lapses.last() {
                            Duration::from_secs(time.as_secs() - last_lap.time.as_secs())
                        } else {
                            time
                        };

                        lapses.push(Lapse { time, delta });

                        scroll_offset = 0;
                    }
                    // Handle scroll in lapses list
                    KeyCode::Down => {
                        scroll_offset = min(scroll_offset + 1, lapses.len());
                    }
                    KeyCode::Up => {
                        scroll_offset = if scroll_offset == 0 {
                            0
                        } else {
                            scroll_offset - 1
                        };
                    }
                    KeyCode::PageDown => {
                        scroll_offset = lapses.len();
                    }
                    KeyCode::PageUp => {
                        scroll_offset = 0;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // Clear frame
        queue!(stdout, terminal::Clear(ClearType::All))?;

        // Render
        render_frame(&config, &chronometer, &lapses, &mut scroll_offset)?;

        config.color.update();

        stdout.flush()?;

        thread::sleep(Duration::from_millis(1000 / config.fps));
    }

    Ok(())
}

fn render_frame(
    config: &Config,
    chronometer: &Chronometer,
    lapses: &Vec<Lapse>,
    scroll_offset: &mut usize,
) -> io::Result<()> {
    let color = config.color.get_value();

    // Display time
    let elapsed = utils::format_duration(chronometer.elapsed());
    rendering::draw_time(&elapsed, color)?;

    // Display lapses
    let (width, height) = rendering::get_terminal_size()?;
    let y = height / 2 + symbols::SYMBOL_HEIGHT as i16 / 2 + 2;
    let max_items = min(10, height - y - 1) as usize;

    if lapses.len() <= max_items {
        *scroll_offset = 0;
    } else if *scroll_offset > lapses.len() - max_items {
        *scroll_offset = lapses.len() - max_items;
    }

    // Iterate over lapses, skipping with scroll offset and taxing N items
    for (i, lapse) in lapses
        .iter()
        .rev()
        .skip(*scroll_offset)
        .take(max_items)
        .enumerate()
    {
        let delta = utils::format_duration(lapse.delta);
        let time = utils::format_duration(lapse.time);

        let lapse = format!(
            "#{:02}  --  +{}  --  {}",
            lapses.len() - i - *scroll_offset,
            delta,
            time
        );
        let x = width / 2 - (lapse.len() as i16) / 2;
        rendering::draw_text(&lapse, x, y + i as i16, color)?;
    }

    // Display pause state
    if chronometer.is_paused() {
        let text = "[PAUSE]";
        let x = width / 2 - (text.len() as i16) / 2 - 1;
        let y = y - symbols::SYMBOL_HEIGHT as i16 + symbols::SYMBOL_HEIGHT as i16 / 2 + 1;

        rendering::draw_text(text, x, y, color)?;
    }

    Ok(())
}
