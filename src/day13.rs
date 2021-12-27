use std::{collections::HashSet, str::FromStr};

use crate::Lines;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(",");

        Ok(Point {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
        })
    }
}

#[derive(Debug)]
enum Dimension {
    X,
    Y,
}

impl FromStr for Dimension {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().unwrap() {
            'x' => Ok(Dimension::X),
            'y' => Ok(Dimension::Y),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Fold {
    dim: Dimension,
    pos: i32,
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ").nth(2).unwrap().split("=");

        Ok(Fold {
            dim: Dimension::from_str(split.next().unwrap()).unwrap(),
            pos: split.next().unwrap().parse().unwrap(),
        })
    }
}

pub struct Solver {}

impl Solver {
    fn solve(self: &mut Self, mut lines: Lines, first: bool) -> String {
        // Parse
        let mut points = HashSet::new();
        loop {
            let line = lines.next().unwrap();

            if line.is_empty() {
                break;
            }

            points.insert(Point::from_str(&line).unwrap());
        }

        let mut folds = Vec::new();
        while let Some(line) = lines.next() {
            folds.push(Fold::from_str(&line).unwrap());
        }

        // Fold
        for Fold { dim, pos } in folds {
            for point in points.clone() {
                let val = match dim {
                    Dimension::X => point.x,
                    Dimension::Y => point.y,
                };

                if val > pos {
                    let new_val = pos - (val - pos);

                    points.remove(&point);

                    match dim {
                        Dimension::X => points.insert(Point {
                            x: new_val,
                            y: point.y,
                        }),
                        Dimension::Y => points.insert(Point {
                            x: point.x,
                            y: new_val,
                        }),
                    };
                }
            }

            if first {
                break;
            }
        }

        // Get result
        if !first {
            let mut max = Point { x: 0, y: 0 };
            for point in points.iter() {
                if point.x > max.x {
                    max.x = point.x;
                }
                if point.y > max.y {
                    max.y = point.y;
                }
            }

            for y in 0..=max.y {
                for x in 0..=max.x {
                    if points.contains(&Point { x, y }) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }

            "written above!".to_string()
        } else {
            points.into_iter().count().to_string()
        }
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
