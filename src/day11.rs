use crate::Lines;

struct Neightbours {
    width: usize,
    height: usize,
    point: (i32, i32),
    current: i32,
}

impl Neightbours {
    pub fn new(width: usize, height: usize, (x, y): (usize, usize)) -> Self {
        Neightbours {
            width,
            height,
            point: (x as i32, y as i32),
            current: 0,
        }
    }
}

impl Iterator for Neightbours {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (x1, y1) = self.point;
        let (x2, y2) = match self.current {
            0 => (x1 - 1, y1),
            1 => (x1 - 1, y1 + 1),
            2 => (x1, y1 + 1),
            3 => (x1 + 1, y1 + 1),
            4 => (x1 + 1, y1),
            5 => (x1 + 1, y1 - 1),
            6 => (x1, y1 - 1),
            7 => (x1 - 1, y1 - 1),
            _ => return None,
        };

        self.current += 1;

        if x2 < 0 || y2 < 0 || y2 >= self.height as i32 || x2 >= self.width as i32 {
            return self.next();
        }

        Some((x2 as usize, y2 as usize))
    }
}

struct MapPoints {
    width: usize,
    height: usize,
    point: (usize, usize),
}

impl MapPoints {
    pub fn new(width: usize, height: usize) -> Self {
        MapPoints {
            width,
            height,
            point: (0, 0),
        }
    }
}

impl Iterator for MapPoints {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.point;
        if y == self.height {
            return None;
        }

        if x == self.width - 1 {
            self.point.0 = 0;
            self.point.1 += 1;
        } else {
            self.point.0 += 1;
        }

        return Some((x, y));
    }
}

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

                    // Increase energy of neightbours that were not flashed yet
                    for (x, y) in Neightbours::new(width, height, (x, y)) {
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
