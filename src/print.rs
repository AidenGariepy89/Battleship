use crate::{general::Player, board::{BOARD_TOTAL, BOARD_WIDTH}};

macro_rules! clearscr {
    () => {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    };
}

pub fn get_input() -> String {
    let mut out = String::new();
    std::io::stdin().read_line(&mut out).expect("Input failed!");
    out
}

pub fn print_board(ships: &[ bool; BOARD_TOTAL ]) {
    // println!("
  // 1 2 3 4 5 6 7 8 9 0
// A . . . . . . . . . .
// B . . . . . . . . . .
// C . . . . . . . . . .
// D
// E
// F
// G
// H
// I
// J
    //          ");
    println!("  1 2 3 4 5 6 7 8 9 0");
    for i in 0..BOARD_WIDTH {
        print!("{} ", ('A' as u8 + i as u8) as char);
        for j in 0..BOARD_WIDTH {
            if ships[(i * BOARD_WIDTH) + j] {
                print!("██");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

pub fn annouce_player(player: Player) {
    match player {
        Player::One => {
            clearscr!();
            println!("





-------------------------------------------------------------------------------

                                Player One

-------------------------------------------------------------------------------




Press any key to continue...");
        }
        Player::Two => {
            clearscr!();
            println!("





-------------------------------------------------------------------------------

                                Player Two

-------------------------------------------------------------------------------




Press any key to continue...
                     ");
        }
    }
}

