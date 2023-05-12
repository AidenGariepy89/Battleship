use std::io;

use battleship::{
    game::{self, GameData, LoopState},
    ui,
};

fn main() -> Result<(), io::Error> {
    // let data = game::setup();
    // let mut state = LoopState::Continue;

    // while let LoopState::Continue = state {
    //     state = game::run(&data);
    // }
    let mut term = ui::setup_terminal()?;

    let data = game::setup(&mut term)?;

    ui::destroy_terminal(term)?;

    Ok(())
}
// std::thread::sleep(std::time::Duration::from_millis(1000));
