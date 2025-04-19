use std::io;

pub fn player_places<F: FnMut(u8, u8)>(_func: F) {
    let mut buf: String = String::new();

    io::stdin().read_line(&mut buf).unwrap();
}
