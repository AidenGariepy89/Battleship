use std::io;

use crossterm::event::{read, Event, KeyCode};

pub enum KeyAction {
    Up,
    Down,
    Left,
    Right,
    Rotate,
    Place,
    Cancel,
    Char(char),
}

pub fn get_key_input() -> io::Result<Option<KeyAction>> {
    if let Event::Key(event) = read()? {
        if let KeyCode::Char(c) = event.code {
            match c {
                'q' => { return Ok(Some(KeyAction::Cancel)); }
                'j' => { return Ok(Some(KeyAction::Down)); }
                'k' => { return Ok(Some(KeyAction::Up)); }
                'h' => { return Ok(Some(KeyAction::Left)); }
                'l' => { return Ok(Some(KeyAction::Right)); }
                'r' => { return Ok(Some(KeyAction::Rotate)); }
                'y' => { return Ok(Some(KeyAction::Place)); }
                _ => { }
            }
        }
    }
    Ok(None)
}

pub fn get_char_input() -> io::Result<Option<KeyAction>> {
    if let Event::Key(event) = read()? {
        if let KeyCode::Char(c) = event.code {
            if c == 'q' {
                return Ok(Some(KeyAction::Cancel));
            }
            if c >= 'a' && c <= 'j' || c >= '0' && c <= '9' {
                return Ok(Some(KeyAction::Char(c)));
            }
        }
    }
    Ok(None)
}

