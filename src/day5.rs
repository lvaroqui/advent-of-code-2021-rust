use std::{collections::HashMap, path::Iter, str::FromStr};

use crate::{parse_line, Lines};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split(',');

        Ok(Point {
            x: i32::from_str(splitted.next().unwrap()).unwrap(),
            y: i32::from_str(splitted.next().unwrap()).unwrap(),
        })
    }
}

#[derive(Debug)]
struct Line {
    from: Point,
    to: Point,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split(' ');

        Ok(Line {
            from: Point::from_str(splitted.next().unwrap()).unwrap(),
            to: Point::from_str(splitted.skip(1).next().unwrap()).unwrap(),
        })
    }
}

struct Range {
    current: i32,
    end: i32,
    finished: bool,
}

impl Range {
    fn new(start: i32, end: i32) -> Self {
        Range {
            current: start,
            end,
            finished: false,
        }
    }
}

impl Iterator for Range {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let res = Some(self.current);

        if self.current == self.end {
            self.finished = true;
        }

        if self.current < self.end {
            self.current += 1;
        } else {
            self.current -= 1;
        }

        res
    }
}

pub struct Solver {}

impl Solver {
    fn solve(self: &Self, lines: Lines, skip_diagonals: bool) -> String {
        let mut points = HashMap::<Point, i32>::new();

        let mut register_point = |x: i32, y: i32| {
            *points.entry(Point { x, y }).or_insert(0) += 1;
        };

        for line in lines {
            let line = parse_line::<Line>(line);

            if line.from.x == line.to.x {
                for y in Range::new(line.from.y, line.to.y) {
                    register_point(line.from.x, y);
                }
            } else if line.from.y == line.to.y {
                for x in Range::new(line.from.x, line.to.x) {
                    register_point(x, line.from.y);
                }
            } else if !skip_diagonals {
                let x_iter = Range::new(line.from.x, line.to.x);
                let y_iter = Range::new(line.from.y, line.to.y);
                for (x, y) in x_iter.zip(y_iter) {
                    register_point(x, y);
                }
            }
        }

        points.iter().filter(|&(_, v)| *v > 1).count().to_string()
    }
}

impl crate::Solver for Solver {
    fn solve_part1(self: &mut Self, lines: Lines) -> String {
        self.solve(lines, true)
    }

    fn solve_part2(self: &mut Self, lines: Lines) -> String {
        self.solve(lines, false)
    }
}
