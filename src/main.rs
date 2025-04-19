mod display;

use display::display_board;

use std::fmt::Display;

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Cross,
    Zero,
}

impl Display for Cell {
    fn display() {}
}

fn play_game() {
    let mut board: [[Cell; 3]; 3] = [[Cell::Empty; 3]; 3];

    loop {
        // hod

        display_board(board);

        // parse_move
    }
}

fn main() {
    loop {
        play_game();
    }
}
