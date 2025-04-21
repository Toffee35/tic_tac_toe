use super::Cell;

pub fn display_board(board: [[Cell; 3]; 3]) {
    println!("\x1B[2J\x1B[1;1H  a b c"); // Отчиска консоли "\x1B[2J", перемещение курсора в начало "x1B[1;1H" и вывод "  a b c"

    let mut string_number: u8 = 1;

    for string in board {
        if string_number < 3 {
            print!("{} \x1b[4m", string_number); // включение подчеркивания
        } else {
            print!("{}", string_number);
        }

        let mut cell_number: u8 = 1;

        for cell in string {
            print!("{}", cell);

            if cell_number < 3 {
                print!("|");
            } else {
                if string_number < 3 {
                    println!("\x1b[0m"); // выключение подчеркивания
                } else {
                    print!("\n");
                }
            }

            cell_number += 1;
        }

        string_number += 1;
    }
}
