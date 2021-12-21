use crate::Lines;

pub struct Solver {}

type Map<T> = Vec<Vec<T>>;

struct Neightbours<'a, T> {
    map: &'a Map<T>,
    point: (i32, i32),
    current: i32,
}

impl<'a, T> Neightbours<'a, T> {
    pub fn new(map: &'a Map<T>, point: (i32, i32)) -> Self {
        Neightbours {
            map,
            point,
            current: 0,
        }
    }
}

impl<'a, T> Iterator for Neightbours<'a, T>
where
    T: Copy,
{
    type Item = ((i32, i32), T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 4 {
            return None;
        }
        let (x, y) = self.point;
        let (x, y) = match self.current {
            0 => (x - 1, y),
            1 => (x, y + 1),
            2 => (x + 1, y),
            3 => (x, y - 1),
            _ => panic!("This should never happens"),
        };

        self.current += 1;

        if x < 0 || y < 0 || y >= self.map.len() as i32 || x >= self.map[y as usize].len() as i32 {
            return self.next();
        }

        Some(((x, y), self.map[y as usize][x as usize]))
    }
}

impl Solver {
    fn compute_basins_size(
        &self,
        map: &Map<i32>,
        visited_map: &mut Map<bool>,
        point: (i32, i32),
    ) -> u32 {
        let mut sum = 0;
        for ((x, y), val) in Neightbours::new(&map, point) {
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

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                let val = map[y][x];
                if Neightbours::new(&map, (x as i32, y as i32)).all(|(_, v)| v > val) {
                    sum += val + 1;
                }
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
                let size = self.compute_basins_size(&map, &mut visited_map, (x as i32, y as i32));
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
