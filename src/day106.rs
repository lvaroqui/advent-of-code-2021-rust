// I really enjoyed day 6 and wanted to use it as a simple example of the
// dangers of spatial and time complexity.

use std::collections::LinkedList;

use crate::Lines;

pub struct Solver {}

impl Solver {
    fn solve(self: &mut Self, mut lines: Lines, days: u32) -> String {
        let mut fishes = lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<LinkedList<_>>();

        for i in 0..days {
            let mut to_add = LinkedList::<u32>::new();

            println!("{}", i);

            for fish in fishes.iter_mut() {
                if *fish == 0 {
                    *fish = 6;
                    to_add.push_back(8);
                } else {
                    *fish -= 1;
                }
            }
            fishes.append(&mut to_add);
        }

        (fishes.len()).to_string()
    }
}

impl crate::Solver for Solver {
    fn solve_part1(self: &mut Self, lines: Lines) -> String {
        self.solve(lines, 80)
    }

    fn solve_part2(self: &mut Self, lines: Lines) -> String {
        self.solve(lines, 256)
    }
}
