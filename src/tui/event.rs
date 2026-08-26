use std::time::Duration;

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Key(KeyEvent),
    Resize(u16, u16),
    Tick,
}

pub fn next_event(tick_rate: Duration) -> std::io::Result<Event> {
    if event::poll(tick_rate)? {
        match event::read()? {
            CrosstermEvent::Key(key) => Ok(Event::Key(key)),
            CrosstermEvent::Resize(width, height) => Ok(Event::Resize(width, height)),
            _ => Ok(Event::Tick),
        }
    } else {
        Ok(Event::Tick)
    }
}
