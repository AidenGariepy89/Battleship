use crate::{general::Marker, placement, ui};

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_TOTAL: usize = 100;

pub struct BoardBuilder<'a> {
    term: &'a mut ui::Term,
    ships: [ bool; BOARD_TOTAL ],
}

pub struct Board {
    marks: [ Option<Marker>; BOARD_TOTAL ],
    ships: [ bool; BOARD_TOTAL ],
}

impl Board {
    pub fn builder<'a>(term: &'a mut ui::Term) -> BoardBuilder<'a> {
        BoardBuilder::new(term)
    }
}

impl<'a> BoardBuilder<'a> {
    fn new(term: &'a mut ui::Term) -> Self {
        BoardBuilder {
            term,
            ships: [ false; BOARD_TOTAL ]
        }
    }
    pub fn add_carrier(mut self) -> BoardBuilder<'a> {
        let ship = placement::place_piece(&self.ships, 5);

        for pos in ship {
            self.ships[pos] = true;
        }

        self
    }
    pub fn add_battleship(mut self) -> BoardBuilder<'a> {
        let ship = placement::place_piece(&self.ships, 4);

        for pos in ship {
            self.ships[pos] = true;
        }

        self
    }
    pub fn add_cruiser(mut self) -> BoardBuilder<'a>{
        let ship = placement::place_piece(&self.ships, 3);

        for pos in ship {
            self.ships[pos] = true;
        }

        self
    }
    pub fn add_submarine(mut self) -> BoardBuilder<'a> {
        let ship = placement::place_piece(&self.ships, 3);

        for pos in ship {
            self.ships[pos] = true;
        }

        self
    }
    pub fn add_destroyer(mut self) -> BoardBuilder<'a> {
        let ship = placement::place_piece(&self.ships, 2);

        for pos in ship {
            self.ships[pos] = true;
        }

        self
    }
    pub fn finish(&self) -> Board {
        Board { marks: [ None; BOARD_TOTAL ], ships: self.ships }
    }
}

