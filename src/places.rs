use std::io::{self, Write};

#[derive(Debug)]
pub enum ParseError {
    LengthLessThan2,
    LengthGreaterThan2,
    ExtraCharacters,
    InvalidFormat,
}

pub fn player_places() -> Result<(u8, u8), ParseError> {
    print!("You (x): ");
    io::stdout().flush().unwrap();

    let mut buf: String = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let chars: Vec<char> = buf.trim().chars().collect();

    match chars.len() {
        0 | 1 => Err(ParseError::LengthLessThan2),
        2 => {
            if !chars.iter().all(|chr: &char| "123abc".contains(*chr)) {
                return Err(ParseError::ExtraCharacters);
            }

            if let (Some(first), Some(second)) = (chars.get(0), chars.get(1)) {
                if "123".contains(*first) && "abc".contains(*second) {
                    return Ok((first.clone() as u8 - b'1', second.clone() as u8 - b'a'));
                }

                if "abc".contains(*first) && "123".contains(*second) {
                    return Ok((second.clone() as u8 - b'1', first.clone() as u8 - b'a'));
                }
            }

            Err(ParseError::InvalidFormat)
        }
        _ => Err(ParseError::LengthGreaterThan2),
    }
}
