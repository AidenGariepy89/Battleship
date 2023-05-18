use std::io;

use crate::{
    board::{Board, BOARD_WIDTH, BOARD_TOTAL},
    general::{Player, Marker},
    ui::{self, Term}, input::{KeyAction, self},
};

pub struct GameData<'a> {
    pub term: &'a mut Term,
    pub player_one: Board,
    pub player_two: Board,
    pub turn: Player,
}

impl<'a> GameData<'a> {
    pub fn next_turn(&mut self) {
        match self.turn {
            Player::One => {
                self.turn = Player::Two;
            }
            Player::Two => {
                self.turn = Player::One;
            }
        }
    }
}

pub enum LoopState {
    Continue,
    Exit,
}

pub fn setup(term: &mut Term) -> io::Result<GameData> {
    ui::render_turn(term, Player::One)?;
    input::get_key_input()?;
    let player_one = Board::builder(term)
        .add_destroyer()
        .add_cruiser()
        .add_submarine()
        .add_battleship()
        .add_carrier()
        .finish();

    ui::render_turn(term, Player::Two)?;
    input::get_key_input()?;
    let player_two = Board::builder(term)
        .add_destroyer()
        .add_cruiser()
        .add_submarine()
        .add_battleship()
        .add_carrier()
        .finish();

    ui::render_turn(term, Player::One)?;

    Ok(GameData { term, player_one, player_two, turn: Player::One })
}

pub fn run(data: &mut GameData) -> io::Result<LoopState> {
    input::get_key_input()?;
    ui::render_board(data)?;

    match take_turn(data)? {
        TurnReturn::Completed => {
            data.next_turn();
            ui::clear(data.term)?;
            ui::render_turn(data.term, data.turn)?;
        }
        TurnReturn::InvalidInput => { }
        TurnReturn::Quit => {
            return Ok(LoopState::Exit);
        }
    }

    if data.player_one.check_for_lose() {
        ui::render_victory(data.term, Player::Two)?;
        input::get_key_input()?;
        return Ok(LoopState::Exit);
    }
    if data.player_two.check_for_lose() {
        ui::render_victory(data.term, Player::One)?;
        input::get_key_input()?;
        return Ok(LoopState::Exit);
    }

    Ok(LoopState::Continue)
}

enum TurnReturn {
    Completed,
    InvalidInput,
    Quit,
}

fn take_turn(data: &mut GameData) -> io::Result<TurnReturn> {
    let result = input::get_char_input()?;
    let mut row = ' ';
    if let Some(KeyAction::Cancel) = result {
        return Ok(TurnReturn::Quit);
    }
    if let Some(KeyAction::Char(c)) = result {
        if c >= 'a' && c <= 'j' {
            row = c;
        } else {
            return Ok(TurnReturn::InvalidInput);
        }
    }
    let result = input::get_char_input()?;
    let mut col = ' ';
    if let Some(KeyAction::Cancel) = result {
        return Ok(TurnReturn::Quit);
    }
    if let Some(KeyAction::Char(c)) = result {
        if c >= '0' && c <= '9' {
            col = c;
        } else {
            return Ok(TurnReturn::InvalidInput);
        }
    }

    if row < 'a' {
        return Ok(TurnReturn::InvalidInput);
    }
    if col != '0' && col < '1' {
        return Ok(TurnReturn::InvalidInput);
    }

    let row = row as usize - 'a' as usize;
    let col = if col == '0' {
        9
    } else {
        col as usize - '1' as usize
    };
    let coord = (row * BOARD_WIDTH) + col as usize;
    if coord >= BOARD_TOTAL {
        return Ok(TurnReturn::InvalidInput);
    }

    match data.turn {
        Player::One => {
            if let Some(_) = data.player_two.marks[coord] {
                return Ok(TurnReturn::InvalidInput);
            }
            if data.player_two.ships[coord] {
                data.player_two.marks[coord] = Some(Marker::Hit);
                ui::render_result(data.term, true)?;
            } else {
                data.player_two.marks[coord] = Some(Marker::Miss);
                ui::render_result(data.term, false)?;
            }
        }
        Player::Two => {
            if let Some(_) = data.player_one.marks[coord] {
                return Ok(TurnReturn::InvalidInput);
            }
            if data.player_one.ships[coord] {
                data.player_one.marks[coord] = Some(Marker::Hit);
                ui::render_result(data.term, true)?;
            } else {
                data.player_one.marks[coord] = Some(Marker::Miss);
                ui::render_result(data.term, false)?;
            }
        }
    }

    input::get_key_input()?;

    Ok(TurnReturn::Completed)
}

