use crate::Lines;

use petgraph::algo::dijkstra;
use petgraph::graphmap::GraphMap;
use petgraph::Directed;

type MyGraph = GraphMap<(usize, usize), u32, Directed>;

struct Neightbours {
    width: usize,
    height: usize,
    point: (i32, i32),
    current: u32,
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
            1 => (x1, y1 + 1),
            2 => (x1 + 1, y1),
            3 => (x1, y1 - 1),
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

type Map = Vec<Vec<u32>>;

impl Solver {
    fn find_shortest_path(&self, map: &Map) -> u32 {
        let mut g = MyGraph::new();

        for (to_x, to_y) in MapPoints::new(map[0].len(), map.len()) {
            for (from_x, from_y) in Neightbours::new(map[0].len(), map.len(), (to_x, to_y)) {
                g.add_edge((from_x, from_y), (to_x, to_y), map[to_y][to_x]);
            }
        }

        let start = (0, 0);
        let goal = (map[0].len() - 1, map.len() - 1);

        let res = dijkstra(&g, start, Some(goal), |(_, _, e)| *e);

        res[&goal]
    }
}

pub struct Solver {}

impl crate::Solver for Solver {
    fn solve_part1(self: &mut Self, lines: Lines) -> String {
        let map: Map = lines
            .map(|line| {
                line.chars()
                    .map(|s| s.to_string().parse().unwrap())
                    .collect()
            })
            .collect::<Vec<_>>();

        self.find_shortest_path(&map).to_string()
    }

    fn solve_part2(self: &mut Self, lines: Lines) -> String {
        let map: Map = lines
            .map(|line| {
                line.chars()
                    .map(|s| s.to_string().parse().unwrap())
                    .collect()
            })
            .collect::<Vec<_>>();

        let mut new_map = Map::new();

        for i in 0..(5 * map.len()) {
            new_map.push(Vec::new());
            for j in 0..5 {
                for v in map[i % map.len()].iter() {
                    let mut new_val = *v as u32 + (i as u32 / map.len() as u32) + j as u32;
                    if new_val > 9 {
                        new_val -= 9;
                    }
                    new_map[i].push(new_val);
                }
            }
        }

        self.find_shortest_path(&new_map).to_string()
    }
}
