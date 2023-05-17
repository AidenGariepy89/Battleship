use std::io;

use crate::{
    board::Board,
    general::Player,
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

    Ok(GameData { term, player_one, player_two, turn: Player::One })
}

pub fn run(data: &mut GameData) -> io::Result<LoopState> {
    ui::render_turn(data.term, data.turn)?;
    input::get_key_input()?;
    ui::render_board(data)?;

    let result = input::get_key_input()?;
    if let Some(KeyAction::Cancel) = result {
        return Ok(LoopState::Exit);
    }

    data.next_turn();

    Ok(LoopState::Continue)
}

