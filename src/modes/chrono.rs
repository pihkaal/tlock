use std::{
    cmp::min,
    io::{self, Write},
    thread,
    time::{self, Duration},
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
    pub time: time::Duration,
    pub delta: time::Duration,
}

pub fn main_loop(config: &mut Config) -> io::Result<()> {
    let mut stdout = io::stdout();

    let start_time = time::Instant::now();
    let mut lapses: Vec<Lapse> = vec![];

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
                    // Handle lapse
                    KeyCode::Char(' ') => {
                        let time = start_time.elapsed();
                        let delta = if let Some(last_lap) = lapses.last() {
                            time::Duration::from_secs(time.as_secs() - last_lap.time.as_secs())
                        } else {
                            time
                        };

                        lapses.push(Lapse { time, delta });
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // Clear frame
        queue!(stdout, terminal::Clear(ClearType::All))?;

        // Render
        render_frame(&config, start_time, &lapses)?;

        config.color.update();

        stdout.flush()?;

        thread::sleep(Duration::from_millis(1000 / config.fps));
    }

    return Ok(());
}

fn render_frame(config: &Config, start_time: time::Instant, lapses: &Vec<Lapse>) -> io::Result<()> {
    let color = config.color.get_value();

    // Display time
    let elapsed = utils::format_duration(start_time.elapsed());
    rendering::draw_time(&elapsed, color)?;

    // Display lapses
    let (width, height) = terminal::size()?;
    let y = height / 2 + symbols::SYMBOL_HEIGHT as u16 / 2 + 2;
    let max_items = min(10, height - y) as usize;

    for (i, lapse) in lapses.iter().rev().take(max_items).enumerate() {
        let delta = utils::format_duration(lapse.delta);
        let time = utils::format_duration(lapse.time);

        let lapse = format!("#0{}  --  +{}  --  {}", lapses.len() - i, delta, time);
        let x = width / 2 - (lapse.len() as u16) / 2;
        rendering::draw_text(&lapse, x, y + i as u16, color)?;
    }

    return Ok(());
}
