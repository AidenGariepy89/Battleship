use std::io;
use battleship::{
    game::{self, LoopState},
    ui,
};

fn main() -> Result<(), io::Error> {
    let mut term = ui::setup_terminal()?;

    let mut data = game::setup(&mut term)?;
    let mut state = LoopState::Continue;

    while let LoopState::Continue = state {
        state = game::run(&mut data);
    }

    ui::destroy_terminal(term)?;

    Ok(())
}

