use std::str::FromStr;

use crate::{parse_line, Lines};

enum Command {
    Forward,
    Down,
    Up,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Command::Forward),
            "down" => Ok(Command::Down),
            "up" => Ok(Command::Up),
            _ => Err(()),
        }
    }
}

struct Line {
    cmd: Command,
    value: u32,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split(' ').collect::<Vec<_>>();
        Ok(Line {
            cmd: Command::from_str(words[0])?,
            value: words[1].parse::<u32>().unwrap(),
        })
    }
}

pub struct Solver {}

impl crate::Solver for Solver {
    fn solve_part1(self: &mut Self, lines: Lines) -> String {
        let mut x = 0;
        let mut d = 0;

        for line in lines.map(parse_line::<Line>) {
            match line.cmd {
                Command::Down => d += line.value,
                Command::Up => d -= line.value,
                Command::Forward => x += line.value,
            }
        }

        (x * d).to_string()
    }

    fn solve_part2(self: &mut Self, lines: Lines) -> String {
        let mut x = 0;
        let mut d = 0;
        let mut aim = 0;

        for line in lines.map(parse_line::<Line>) {
            match line.cmd {
                Command::Down => aim += line.value,
                Command::Up => aim -= line.value,
                Command::Forward => {
                    x += line.value;
                    d += aim * line.value;
                }
            }
        }

        (x * d).to_string()
    }
}
