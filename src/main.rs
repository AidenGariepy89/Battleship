use battleship::game::{GameData, LoopState, self};

fn main() {
    let data = game::setup();
    let mut state = LoopState::Continue;

    while let LoopState::Continue = state {
        state = game::run(&data);
    }
}

