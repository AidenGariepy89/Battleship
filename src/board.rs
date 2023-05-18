use tui::{
    style::{Color, Style},
    text::{Span, Spans},
};

use crate::{
    game::GameData,
    general::{Marker, Player},
    placement, ui,
};

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_TOTAL: usize = 100;

pub struct BoardBuilder<'a> {
    term: &'a mut ui::Term,
    ships: [bool; BOARD_TOTAL],
}

pub struct Board {
    pub marks: [Option<Marker>; BOARD_TOTAL],
    pub ships: [bool; BOARD_TOTAL],
}

impl Board {
    pub fn builder<'a>(term: &'a mut ui::Term) -> BoardBuilder<'a> {
        BoardBuilder::new(term)
    }
    pub fn check_for_lose(&self) -> bool {
        let mut ships_down = 0;
        for (coord, is_ship) in self.ships.iter().enumerate() {
            if *is_ship {
                if let Some(Marker::Hit) = self.marks[coord] {
                    ships_down += 1;
                }
            }
        }

        if ships_down == 17 {
            return true;
        }
        false
    }
}

impl<'a> BoardBuilder<'a> {
    fn new(term: &'a mut ui::Term) -> Self {
        BoardBuilder {
            term,
            ships: [false; BOARD_TOTAL],
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
    pub fn add_cruiser(mut self) -> BoardBuilder<'a> {
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
        Board {
            marks: [None; BOARD_TOTAL],
            ships: self.ships,
        }
    }
}

pub fn draw_board<'a>(data: &GameData<'a>) -> (Vec<Spans<'a>>, Vec<Spans<'a>>) {
    let mut screen1 = Vec::new();
    let mut screen2 = Vec::new();
    screen1.push(Spans::from(" 1 2 3 4 5 6 7 8 9 0"));
    screen2.push(Spans::from(" 1 2 3 4 5 6 7 8 9 0"));
    match data.turn {
        Player::One => {
            for i in 0..BOARD_WIDTH {
                let mut b1 = Vec::new();
                let mut b2 = Vec::new();
                b1.push(Span::from(
                    (('A' as u8 + i as u8) as char).to_string() + " ",
                ));
                b2.push(Span::from(
                    (('A' as u8 + i as u8) as char).to_string() + " ",
                ));
                for j in 0..BOARD_WIDTH {
                    if let Some(mark) = data.player_one.marks[(i * BOARD_WIDTH) + j] {
                        match mark {
                            Marker::Miss => {
                                b2.push(Span::styled("X ", Style::default().fg(Color::Green)));
                            }
                            Marker::Hit => {
                                b2.push(Span::styled("# ", Style::default().fg(Color::Red)));
                            }
                        }
                    } else {
                        if data.player_one.ships[(i * BOARD_WIDTH) + j] {
                            b2.push(Span::raw("██"));
                        } else {
                            b2.push(Span::styled(". ", Style::default().fg(Color::Gray)));
                        }
                    }
                    if let Some(mark) = data.player_two.marks[(i * BOARD_WIDTH) + j] {
                        match mark {
                            Marker::Miss => {
                                b1.push(Span::styled("X ", Style::default().fg(Color::Green)));
                            }
                            Marker::Hit => {
                                b1.push(Span::styled("# ", Style::default().fg(Color::Red)));
                            }
                        }
                    } else {
                        b1.push(Span::styled(". ", Style::default().fg(Color::Gray)));
                    }
                }
                screen1.push(Spans::from(b1));
                screen2.push(Spans::from(b2));
            }
        }
        Player::Two => {
            for i in 0..BOARD_WIDTH {
                let mut b1 = Vec::new();
                let mut b2 = Vec::new();
                b1.push(Span::from(
                    (('A' as u8 + i as u8) as char).to_string() + " ",
                ));
                b2.push(Span::from(
                    (('A' as u8 + i as u8) as char).to_string() + " ",
                ));
                for j in 0..BOARD_WIDTH {
                    if let Some(mark) = data.player_two.marks[(i * BOARD_WIDTH) + j] {
                        match mark {
                            Marker::Miss => {
                                b2.push(Span::styled("X ", Style::default().fg(Color::Green)));
                            }
                            Marker::Hit => {
                                b2.push(Span::styled("# ", Style::default().fg(Color::Red)));
                            }
                        }
                    } else {
                        if data.player_two.ships[(i * BOARD_WIDTH) + j] {
                            b2.push(Span::raw("██"));
                        } else {
                            b2.push(Span::raw(". "));
                        }
                    }
                    if let Some(mark) = data.player_one.marks[(i * BOARD_WIDTH) + j] {
                        match mark {
                            Marker::Miss => {
                                b1.push(Span::styled("X ", Style::default().fg(Color::Green)));
                            }
                            Marker::Hit => {
                                b1.push(Span::styled("# ", Style::default().fg(Color::Red)));
                            }
                        }
                    } else {
                        b1.push(Span::raw(". "));
                    }
                }
                screen1.push(Spans::from(b1));
                screen2.push(Spans::from(b2));
            }
        }
    }
    (screen1, screen2)
}
