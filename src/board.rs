use crate::general::Marker;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_TOTAL: usize = 100;

pub struct Board {
    marks: [ Option<Marker>; BOARD_TOTAL ],
    ships: [ bool; BOARD_TOTAL ],
}
