use std::io;

#[derive(Debug)]
pub enum ParseError {
    LengthGreaterThan3,
}

pub fn player_places() -> Result<(u8, u8), ParseError> {
    print!("Ваш ход (x): ");

    let mut buf: String = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    let input: String = buf
        .chars()
        .filter(|char| "123abc".contains(*char))
        .collect();

    if input.len() < 3 {
        println!("{}", input);

        Ok((1, 1))
    } else {
        Err(ParseError::LengthGreaterThan3)
    }
}
