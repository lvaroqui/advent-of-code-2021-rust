use crate::Lines;

use crate::utils::{MapPoints, NeightborsNoDiag};

pub struct Solver {}

type Map<T> = Vec<Vec<T>>;

impl Solver {
    fn compute_basins_size(
        &self,
        map: &Map<i32>,
        visited_map: &mut Map<bool>,
        point: (usize, usize),
    ) -> u32 {
        let mut sum = 0;
        for (x, y) in NeightborsNoDiag::new(map[0].len(), map.len(), point) {
            let val = map[y][x];
            if val == 9 || visited_map[y as usize][x as usize] {
                continue;
            }
            visited_map[y as usize][x as usize] = true;
            sum += 1 + self.compute_basins_size(map, visited_map, (x, y));
        }
        sum
    }
}

impl crate::Solver for Solver {
    fn solve_part1(self: &mut Self, lines: Lines) -> String {
        let map: Vec<Vec<i32>> = lines
            .map(|line| {
                line.chars()
                    .map(|s| s.to_string().parse::<i32>().unwrap())
                    .collect()
            })
            .collect::<Vec<_>>();

        let mut sum = 0;

        for (x, y) in MapPoints::new(map[0].len(), map.len()) {
            let val = map[y][x];
            if NeightborsNoDiag::new(map[0].len(), map.len(), (x, y)).all(|(x, y)| map[y][x] > val)
            {
                sum += val + 1;
            }
        }

        sum.to_string()
    }

    fn solve_part2(self: &mut Self, lines: Lines) -> String {
        let map: Vec<Vec<i32>> = lines
            .map(|line| {
                line.chars()
                    .map(|s| s.to_string().parse::<i32>().unwrap())
                    .collect()
            })
            .collect::<Vec<_>>();

        let mut visited_map: Vec<Vec<bool>> = Vec::with_capacity(map.len());
        for _ in 0..map[0].len() {
            let mut vec = Vec::with_capacity(map[0].len());
            for _ in 0..map[0].len() {
                vec.push(false);
            }
            visited_map.push(vec);
        }

        let mut basins = Vec::new();

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                let size = self.compute_basins_size(&map, &mut visited_map, (x, y));
                if size > 0 {
                    basins.push(size);
                }
            }
        }

        basins.sort();
        let res = basins
            .into_iter()
            .rev()
            .take(3)
            .reduce(|acc, val| acc * val)
            .unwrap();

        res.to_string()
    }
}
