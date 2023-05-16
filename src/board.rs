use crate::{general::Marker, placement, ui};

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_TOTAL: usize = 100;

pub struct BoardBuilder<'a> {
    term: &'a mut ui::Term,
    ships: [ bool; BOARD_TOTAL ],
}

pub struct Board {
    pub marks: [ Option<Marker>; BOARD_TOTAL ],
    pub ships: [ bool; BOARD_TOTAL ],
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
        let ship = placement::place_piece(&self.ships, &mut self.term, 5).unwrap();

        for pos in ship {
            self.ships[pos] = true;
        }

        self
    }
    pub fn add_battleship(mut self) -> BoardBuilder<'a> {
        let ship = placement::place_piece(&self.ships, &mut self.term, 4).unwrap();

        for pos in ship {
            self.ships[pos] = true;
        }

        self
    }
    pub fn add_cruiser(mut self) -> BoardBuilder<'a>{
        let ship = placement::place_piece(&self.ships, &mut self.term, 3).unwrap();

        for pos in ship {
            self.ships[pos] = true;
        }

        self
    }
    pub fn add_submarine(mut self) -> BoardBuilder<'a> {
        let ship = placement::place_piece(&self.ships, &mut self.term, 3).unwrap();

        for pos in ship {
            self.ships[pos] = true;
        }

        self
    }
    pub fn add_destroyer(mut self) -> BoardBuilder<'a> {
        let ship = placement::place_piece(&self.ships, &mut self.term, 2).unwrap();

        for pos in ship {
            self.ships[pos] = true;
        }

        self
    }
    pub fn finish(&self) -> Board {
        let mut marks = [ None; BOARD_TOTAL ];
        marks[0] = Some(Marker::Hit);
        marks[1] = Some(Marker::Miss);
        Board { marks, ships: self.ships }
    }
}

