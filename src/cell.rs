use std::fmt::{Display, Error, Formatter};

#[derive(PartialEq, Clone, Copy)]
pub enum Cell {
    Empty,
    Cross,
    Zero,
}

impl Display for Cell {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            formatter,
            "{}",
            match self {
                Cell::Empty => " ",
                Cell::Cross => "x",
                Cell::Zero => "o",
            }
        )
    }
}
