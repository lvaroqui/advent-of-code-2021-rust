use std::fs::File;
use std::io::{self, BufReader};
use std::str::FromStr;

mod bits;
mod utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

mod day106;

pub trait Solver {
    fn solve_part1(self: &mut Self, _lines: Lines) -> String {
        "".to_string()
    }

    fn solve_part2(self: &mut Self, _lines: Lines) -> String {
        "".to_string()
    }
}

struct DummySolver {}

impl Solver for DummySolver {
    fn solve_part1(self: &mut Self, _lines: Lines) -> String {
        "yet to be implemented".to_string()
    }

    fn solve_part2(self: &mut Self, _lines: Lines) -> String {
        "yet to be implemented".to_string()
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

pub fn parse_line<T>(line: String) -> T
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    T::from_str(&line).unwrap()
}

pub fn get_solver(day: u32) -> Box<dyn Solver> {
    match day {
        1 => Box::new(day1::Solver {}),
        2 => Box::new(day2::Solver {}),
        3 => Box::new(day3::Solver {}),
        4 => Box::new(day4::Solver {}),
        5 => Box::new(day5::Solver {}),
        6 => Box::new(day6::Solver {}),
        7 => Box::new(day7::Solver {}),
        8 => Box::new(day8::Solver {}),
        9 => Box::new(day9::Solver {}),
        10 => Box::new(day10::Solver {}),
        11 => Box::new(day11::Solver {}),
        12 => Box::new(day12::Solver::default()),
        13 => Box::new(day13::Solver {}),
        14 => Box::new(day14::Solver {}),
        15 => Box::new(day15::Solver {}),
        16 => Box::new(day16::Solver {}),
        17 => Box::new(day17::Solver {}),
        106 => Box::new(day106::Solver {}),
        _ => Box::new(DummySolver {}),
    }
}
