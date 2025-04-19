use super::Cell;

pub fn display_board(board: [[Cell; 3]; 3]) {
    println!("  a. b. c.");

    let mut str_nomber: u8 = 1;

    for string in board {
        println!("{} {:?}", str_nomber, string);

        str_nomber += 1;
    }
}
