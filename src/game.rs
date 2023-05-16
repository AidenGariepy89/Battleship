use std::io;

use crate::{
    board::Board,
    general::Player,
    ui::{self, Term},
};

pub struct GameData<'a> {
    term: &'a mut Term,
    player_one: Board,
    player_two: Board,
}

pub enum LoopState {
    Continue,
    Exit,
}

pub fn setup(term: &mut Term) -> io::Result<GameData> {
    ui::render_turn(term, Player::One)?;
    let player_one = Board::builder(term)
        .add_destroyer()
        .add_cruiser()
        .add_submarine()
        .add_battleship()
        .add_carrier()
        .finish();

    ui::render_turn(term, Player::Two)?;
    let player_two = Board::builder(term)
        .add_destroyer()
        .add_cruiser()
        .add_submarine()
        .add_battleship()
        .add_carrier()
        .finish();

    Ok(GameData { term, player_one, player_two })
}

pub fn run(data: &mut GameData) -> LoopState {
    println!("Run function!");

    LoopState::Exit
}
