use std::io;

// pub mod eight;
// pub mod eleven;
// pub mod five;
pub mod four;
// pub mod nine;
pub mod one;
// pub mod seven;
// pub mod six;
// pub mod ten;
pub mod three;
pub mod two;

pub struct Input(io::Lines<io::StdinLock<'static>>);

impl Iterator for Input {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|r| r.expect("io error"))
    }
}

pub fn read_input() -> Input {
    Input(io::stdin().lines())
}
