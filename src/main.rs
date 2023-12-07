use std::{env::args, fmt::Display, process, time::Instant};

macro_rules! generate_runner {
    ($(($n:literal, $m:ident)),+) => {
        fn run(problem: &str, input: aoc::Input) -> Result<Box<dyn Display>, String> {
            match problem {
                $(
                    concat!($n,1) => Ok(Box::new(aoc::$m::solve1(input))),
                    concat!($n,2) => Ok(Box::new(aoc::$m::solve2(input))),
                )+
                _ => Err(format!("invalid problem number: {}", problem)),
            }
        }
    };
}

generate_runner!(
    (1, one),
    (2, two),
    (3, three),
    (4, four),
    (5, five),
    (6, six),
    (7, seven)
);

fn main() {
    let args: Vec<_> = args().collect();
    let Some(problem) = args.get(1) else {
        eprintln!("expected a problem number");
        process::exit(1);
    };
    let input = aoc::read_input();
    let start = Instant::now();
    match run(problem, input) {
        Ok(solution) => {
            println!("{}", solution);
            println!("took: {:#?}", Instant::now().duration_since(start));
        }
        Err(e) => eprintln!("error: {}", e),
    }
}
