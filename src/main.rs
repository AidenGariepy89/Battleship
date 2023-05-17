use std::io;
use battleship::{
    game::{self, LoopState},
    ui,
};

fn main() -> Result<(), io::Error> {
    let mut term = ui::setup_terminal()?;

    let mut data = game::setup(&mut term)?;
    data.player_one.marks[35] = Some(battleship::general::Marker::Hit);
    data.player_one.marks[78] = Some(battleship::general::Marker::Miss);
    data.player_two.marks[21] = Some(battleship::general::Marker::Hit);
    data.player_two.marks[59] = Some(battleship::general::Marker::Miss);
    let mut state = LoopState::Continue;

    while let LoopState::Continue = state {
        state = game::run(&mut data)?;
    }

    ui::destroy_terminal(term)?;

    Ok(())
}

