use std::io;

use crossterm::event::{self, read, Event, KeyCode};

pub enum KeyAction {
    Up,
    Down,
    Left,
    Right,
    Rotate,
    Place,
    Cancel,
}

pub fn get_key_input() -> io::Result<Option<KeyAction>> {
    if let Event::Key(event) = read()? {
        if event == KeyCode::Char('q').into() {
            return Ok(Some(KeyAction::Cancel));
        }
    }
    Ok(None)
}

