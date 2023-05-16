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
}

pub fn get_key_input() -> io::Result<Option<KeyAction>> {
    if let Event::Key(event) = read()? {
        if event == KeyCode::Char('q').into() {
            return Ok(Some(KeyAction::Cancel));
        }
        if event == KeyCode::Char('j').into() {
            return Ok(Some(KeyAction::Down));
        }
        if event == KeyCode::Char('k').into() {
            return Ok(Some(KeyAction::Up));
        }
        if event == KeyCode::Char('h').into() {
            return Ok(Some(KeyAction::Left));
        }
        if event == KeyCode::Char('l').into() {
            return Ok(Some(KeyAction::Right));
        }
        if event == KeyCode::Char('r').into() {
            return Ok(Some(KeyAction::Rotate));
        }
        if event == KeyCode::Char('y').into() {
            return Ok(Some(KeyAction::Place));
        }
    }
    Ok(None)
}

