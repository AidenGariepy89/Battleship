use std::io;

use crate::{
    board::{Board, BoardBuilder},
    general::Player,
    placement,
    print::{self, get_input},
    ui::{self, Term},
};

pub struct GameData<'a> {
    term: &'a mut Term,
}

pub enum LoopState {
    Continue,
    Exit,
}

pub fn setup(term: &mut Term) -> io::Result<GameData> {
    ui::render_turn(term, Player::One)?;

    let player_one_board = Board::builder(term)
        .add_destroyer()
        .add_cruiser()
        .add_submarine()
        .add_battleship()
        .add_carrier()
        .finish();

    Ok(GameData { term })
}

pub fn run(data: &GameData) -> LoopState {
    println!("Run function!");

    LoopState::Exit
}
