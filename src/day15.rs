use crate::Lines;

use crate::utils::{MapPoints, NeightborsNoDiag};

use petgraph::algo::dijkstra;
use petgraph::graphmap::GraphMap;
use petgraph::Directed;

type MyGraph = GraphMap<(usize, usize), u32, Directed>;

type Map = Vec<Vec<u32>>;

impl Solver {
    fn find_shortest_path(&self, map: &Map) -> u32 {
        let mut g = MyGraph::new();

        for (to_x, to_y) in MapPoints::new(map[0].len(), map.len()) {
            for (from_x, from_y) in NeightborsNoDiag::new(map[0].len(), map.len(), (to_x, to_y)) {
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
