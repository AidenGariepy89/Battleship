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

    // let ships = [false; battleship::board::BOARD_TOTAL];
    // ui::render_board(&mut term, &ships)?;

    // battleship::input::get_key_input()?;

    let data = game::setup(&mut term)?;

    ui::destroy_terminal(term)?;

    Ok(())
}
// std::thread::sleep(std::time::Duration::from_millis(1000));

fn test() -> io::Result<()> {
    use battleship::input;
    use battleship::input::KeyAction;
    loop {
        let result = input::get_key_input()?;
        if let Some(key) = result {
            match key {
                KeyAction::Up => println!("Up"),
                KeyAction::Down => println!("Down"),
                KeyAction::Right => println!("Right"),
                KeyAction::Left => println!("Right"),
                _ => { break; }
            }
        }
    }
    Ok(())
}

