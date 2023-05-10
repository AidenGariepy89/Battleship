
pub struct GameData {

}

pub enum LoopState {
    Continue,
    Exit,
}

pub fn run(data: &GameData) -> LoopState {
    println!("Run function!");

    LoopState::Exit
}

impl GameData {
    pub fn new() -> Self {
        GameData { }
    }
}
