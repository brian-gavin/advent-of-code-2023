use std::io;

// pub mod eight;
// pub mod eleven;
// pub mod five;
// pub mod four;
// pub mod nine;
pub mod one;
// pub mod seven;
// pub mod six;
// pub mod ten;
// pub mod three;
pub mod two;

pub fn read_input() -> io::Result<Vec<String>> {
    io::stdin().lines().collect()
}
