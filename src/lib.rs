use std::fs::File;
use std::io::{self, BufReader};

mod day1;

pub trait Solver {
    fn solve(self: &mut Self, _lines: Lines) -> String {
        "".to_string()
    }
}

pub struct Lines {
    lines: io::Lines<BufReader<File>>,
}

impl Lines {
    pub fn new(lines: io::Lines<BufReader<File>>) -> Self {
        Lines { lines: lines }
    }
}

impl Iterator for Lines {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            Some(Ok(n)) => Some(n),
            _ => None,
        }
    }
}

pub fn get_solver(day: u32, part: u32) -> Box<dyn Solver> {
    match (day, part) {
        (1, 1) => Box::new(day1::Solver1 {}),
        (1, 2) => Box::new(day1::Solver2 {}),
        _ => panic!("Day {} part {} is not implemented yet", day, part),
    }
}
