use std::{
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

struct Timer {
    duration: Duration,
    end_time: Option<Instant>,
    paused_duration: Duration,
}

impl Timer {
    fn new(duration: Duration) -> Self {
        let end_time = Some(Instant::now() + duration + Duration::from_secs(1));
        Timer {
            duration,
            end_time,
            paused_duration: Duration::ZERO,
        }
    }

    fn toggle_pause(&mut self) {
        if let Some(end_time) = self.end_time {
            self.paused_duration += end_time.duration_since(Instant::now());
            self.end_time = None;
        } else {
            self.end_time = Some(Instant::now() + self.paused_duration);
            self.paused_duration = Duration::ZERO;
        }
    }

    fn time_left(&self) -> Duration {
        if let Some(end_time) = self.end_time {
            let remaining_time = if Instant::now() < end_time {
                end_time.duration_since(Instant::now())
            } else {
                Duration::ZERO
            };

            remaining_time - self.paused_duration
        } else {
            self.paused_duration
        }
    }

    fn is_finished(&self) -> bool {
        self.time_left().as_secs() == 0
    }

    fn is_paused(&self) -> bool {
        return self.end_time.is_none();
    }

    fn reset(&mut self) {
        self.end_time = Some(Instant::now() + self.duration + Duration::from_secs(1));
        self.paused_duration = Duration::ZERO;
        self.toggle_pause();
    }
}

pub fn main_loop(config: &mut Config, duration: &str) -> io::Result<()> {
    let mut stdout = io::stdout();

    let duration = parse_duration::parse(duration).unwrap();
    let mut timer = Timer::new(duration);

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
                        timer.toggle_pause();
                    }
                    // Handle reset
                    KeyCode::Char('r') => {
                        timer.reset();
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // Clear frame
        queue!(stdout, terminal::Clear(ClearType::All))?;

        // Render
        render_frame(&config, &timer)?;

        config.color.update();

        stdout.flush()?;

        thread::sleep(Duration::from_millis(1000 / config.fps));
    }

    return Ok(());
}

fn render_frame(config: &Config, timer: &Timer) -> io::Result<()> {
    let color = config.color.get_value();

    // Display time
    let remaining = utils::format_duration(timer.time_left());
    rendering::draw_time(&remaining, color)?;

    // Display pause state
    let (width, height) = rendering::get_terminal_size()?;
    let y = height / 2 + symbols::SYMBOL_HEIGHT as i16 / 2 + 2;
    if timer.is_paused() {
        let text = "[PAUSE]";
        let x = width / 2 - (text.len() as i16) / 2;

        rendering::draw_text(
            text,
            x,
            y - symbols::SYMBOL_HEIGHT as i16 - symbols::SYMBOL_HEIGHT as i16 / 2,
            color,
        )?;
    }
    // Display finish state
    else if timer.is_finished() {
        let text = "[FINISHED]";
        let x = width / 2 - (text.len() as i16) / 2;

        rendering::draw_text(
            text,
            x,
            y - symbols::SYMBOL_HEIGHT as i16 - symbols::SYMBOL_HEIGHT as i16 / 2,
            color,
        )?;
    }

    return Ok(());
}
