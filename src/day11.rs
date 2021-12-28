use crate::Lines;

use crate::utils::{MapPoints, NeightborsDiag};

pub struct Solver {}

impl Solver {
    fn solve<F>(&self, lines: Lines, stop_condition: F) -> String
    where
        F: Fn(usize, usize, bool) -> Option<String>,
    {
        let mut octopuses: Vec<Vec<i32>> = lines
            .map(|line| {
                line.chars()
                    .map(|s| s.to_string().parse::<i32>().unwrap())
                    .collect()
            })
            .collect::<Vec<_>>();

        let height = octopuses.len();
        let width = octopuses[0].len();

        let mut flashed: Vec<Vec<bool>> = Vec::with_capacity(octopuses.len());
        for _ in 0..height {
            let mut vec = Vec::with_capacity(octopuses[0].len());
            for _ in 0..width {
                vec.push(false);
            }
            flashed.push(vec);
        }

        let mut flashes = 0;

        let mut step = 0;

        loop {
            step += 1;

            // Increment energy of all octopuses by one
            for (x, y) in MapPoints::new(width, height) {
                flashed[y][x] = false;
                octopuses[y][x] += 1;
            }

            let mut keep_going = true;

            while keep_going {
                keep_going = false;
                for (x, y) in MapPoints::new(width, height) {
                    if octopuses[y][x] <= 9 {
                        continue;
                    }

                    // Flash octopuses above 9 energy
                    flashes += 1;
                    octopuses[y][x] = 0;
                    flashed[y][x] = true;

                    // Increase energy of NeightboursDiag that were not flashed yet
                    for (x, y) in NeightborsDiag::new(width, height, (x, y)) {
                        if flashed[y][x] {
                            continue;
                        }
                        octopuses[y][x] += 1;

                        // If at least one octopus went above 9, we know we need to do another pass
                        if octopuses[y][x] > 9 {
                            keep_going = true;
                        }
                    }
                }
            }

            let all_flashed = MapPoints::new(width, height).all(|(x, y)| flashed[y][x]);

            if let Some(ret) = stop_condition(step, flashes, all_flashed) {
                return ret;
            }
        }
    }
}

impl crate::Solver for Solver {
    fn solve_part1(self: &mut Self, lines: Lines) -> String {
        self.solve(lines, |step, flashes, _| {
            if step == 100 {
                Some(flashes.to_string())
            } else {
                None
            }
        })
    }

    fn solve_part2(self: &mut Self, lines: Lines) -> String {
        self.solve(lines, |step, _, all_flashed| {
            if all_flashed {
                Some(step.to_string())
            } else {
                None
            }
        })
    }
}
