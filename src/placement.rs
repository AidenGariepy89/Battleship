use crate::{
    board::{BOARD_TOTAL, BOARD_WIDTH},
    input::{self, KeyAction},
    ui::{self, Term},
};

fn init_place(length: usize) -> Vec<usize> {
    let mut ship = Vec::<usize>::with_capacity(length);

    let mid = (BOARD_TOTAL / 2) + (BOARD_WIDTH / 2) - 1;
    if length % 2 == 0 {
        ship.push(mid);
        ship.push(mid + 1);

        let mut far = 1;
        for _ in 0..((length - 2) / 2) {
            ship.push(mid - far);
            ship.push(mid + far + 1);
            far += 1;
        }
    } else {
        ship.push(mid);

        let mut far = 1;
        for _ in 0..((length - 1) / 2) {
            ship.push(mid - far);
            ship.push(mid + far);
            far += 1;
        }
    }

    ship
}

pub fn place_piece(ships: &[bool; BOARD_TOTAL], term: &mut Term, length: usize) -> std::io::Result<Vec<usize>> {
    assert!(length != 0);

    let mut ship = init_place(length);

    loop {
        let mut board = *ships;
        for pos in &ship {
            board[*pos] = true;
        }

        ui::render_clean_board(term, &board)?;

        let result = input::get_key_input()?;
        if let Some(key) = result {
            match key {
                KeyAction::Down => {
                    let mut can_move = true;
                    for pos in ship.iter() {
                        if *pos >= BOARD_TOTAL - BOARD_WIDTH {
                            can_move = false;
                        }
                    }
                    if can_move {
                        for pos in ship.iter_mut() {
                            *pos += BOARD_WIDTH;
                        }
                    }
                }
                KeyAction::Up => {
                    let mut can_move = true;
                    for pos in ship.iter() {
                        if *pos < BOARD_WIDTH {
                            can_move = false;
                        }
                    }
                    if can_move {
                        for pos in ship.iter_mut() {
                            *pos -= BOARD_WIDTH;
                        }
                    }
                }
                KeyAction::Right => {
                    let mut can_move = true;
                    for pos in ship.iter() {
                        if *pos % BOARD_WIDTH >= BOARD_WIDTH - 1 {
                            can_move = false;
                        }
                    }
                    if can_move {
                        for pos in ship.iter_mut() {
                            *pos += 1;
                        }
                    }
                }
                KeyAction::Left => {
                    let mut can_move = true;
                    for pos in ship.iter() {
                        if *pos % BOARD_WIDTH == 0 {
                            can_move = false;
                        }
                    }
                    if can_move {
                        for pos in ship.iter_mut() {
                            *pos -= 1;
                        }
                    }
                }
                KeyAction::Place => {
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(ship)
}
