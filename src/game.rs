use crate::{print::{self, get_input}, general::Player, board::{BoardBuilder, Board}, placement};

pub struct GameData {

}

pub enum LoopState {
    Continue,
    Exit,
}

pub fn setup() -> GameData {
    print::annouce_player(Player::One);
    let _ = get_input();

    clearscr!();
    let player_one_board = Board::builder()
        .add_destroyer()
        .add_cruiser()
        .add_submarine()
        .add_battleship()
        .add_carrier()
        .finish();

    GameData { }
}

pub fn run(data: &GameData) -> LoopState {
    println!("Run function!");

    LoopState::Exit
}

