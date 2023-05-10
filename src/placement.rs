use crate::{
    board::{BOARD_TOTAL, BOARD_WIDTH},
    print::{self, get_input},
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

pub fn place_piece(board: &[bool; BOARD_TOTAL], length: usize) -> Vec<usize> {
    assert!(length != 0);

    let mut ship = init_place(length);

    loop {
        let mut board = *board;
        for pos in &ship {
            board[*pos] = true;
        }
        clearscr!();
        print::print_board(&board);

        let input = get_input().to_lowercase();
        let input = input.trim();

        match input {
            "j" => {
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
            "k" => {
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
            "l" => {
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
            "h" => {
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
            "r" => {
            }
            "y" => {
                break;
            }
            _ => {}
        }
    }

    ship
}
