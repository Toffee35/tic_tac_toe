mod cell;
mod check;
mod display;
mod places;

use cell::Cell;
use check::check_winner;
use display::display_board;
use places::player_places;

fn play_game() {
    let mut board: [[Cell; 3]; 3] = [[Cell::Empty; 3]; 3];

    let mut is_over: bool = false;

    while !is_over {
        display_board(board);

        player_places()
            .map(|(x, y)| {
                board[x as usize][y as usize] = Cell::Cross;
            })
            .unwrap();

        is_over = check_winner(board);
    }
}

fn main() {
    loop {
        play_game();
    }
}
