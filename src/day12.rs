use crate::Lines;

use petgraph::graph::NodeIndex;
use petgraph::{Graph, Undirected};

pub struct Solver {
    start: NodeIndex,
    end: NodeIndex,
    small_cave: Vec<bool>,
}

impl Default for Solver {
    fn default() -> Self {
        Self {
            start: Default::default(),
            end: Default::default(),
            small_cave: Default::default(),
        }
    }
}

type MyGraph = Graph<String, (), Undirected>;

impl Solver {
    fn parse_graph(&self, lines: Lines) -> MyGraph {
        let mut g = MyGraph::new_undirected();

        for line in lines {
            let mut edge = [NodeIndex::default(); 2];
            for (n, node) in line.split('-').enumerate() {
                let index;
                if let Some(i) = g.node_indices().find(|i| g[*i] == node) {
                    index = i;
                } else {
                    index = g.add_node(node.to_string());
                }
                edge[n] = index;
            }
            g.add_edge(edge[0], edge[1], ());
        }

        g
    }

    fn init(&mut self, g: &MyGraph) {
        self.start = self.find_node_index(&g, "start");
        self.end = self.find_node_index(&g, "end");

        self.small_cave = vec![false; g.node_count()];
        g.node_indices()
            .for_each(|i| self.small_cave[i.index()] = g[i].chars().any(|c| c.is_lowercase()));
    }

    fn find_node_index(&self, g: &MyGraph, name: &str) -> NodeIndex {
        g.node_indices().find(|i| g[*i] == name).unwrap()
    }

    fn is_cave_small(&self, index: NodeIndex) -> bool {
        self.small_cave[index.index()]
    }

    fn count_path(&self, n: NodeIndex, g: &MyGraph, visited: &mut Vec<bool>) -> u32 {
        let mut sum = 0;

        if n == self.end {
            return 1;
        }

        visited[n.index()] = self.is_cave_small(n);

        for other in g.neighbors(n) {
            if visited[other.index()] {
                continue;
            }
            sum += self.count_path(other, &g, &mut visited.clone());
        }

        sum
    }

    fn count_path_2(
        &self,
        n: NodeIndex,
        mut visited_twice: bool,
        g: &MyGraph,
        visited: &mut Vec<u32>,
    ) -> u32 {
        let mut sum = 0;

        if n == self.end {
            return 1;
        }

        if self.is_cave_small(n) {
            visited[n.index()] += 1;
            if visited[n.index()] == 2 {
                visited_twice = true;
            }
        }

        for other in g.neighbors(n) {
            if other == self.start {
                continue;
            }

            if visited[other.index()] >= 1 && visited_twice {
                continue;
            }

            // Save allocations by avoiding unnecessary clone when cave is big
            let mut cloned = if self.is_cave_small(other) {
                Some(visited.clone())
            } else {
                None
            };

            sum += self.count_path_2(
                other,
                visited_twice,
                &g,
                if let Some(v) = cloned.as_mut() {
                    v
                } else {
                    visited
                },
            );
        }

        sum
    }
}

impl crate::Solver for Solver {
    fn solve_part1(self: &mut Self, lines: Lines) -> String {
        let g = self.parse_graph(lines);

        self.init(&g);

        let mut vec = vec![false; g.node_count()];
        vec[self.start.index()] = true;

        self.count_path(self.start, &g, &mut vec).to_string()
    }

    fn solve_part2(self: &mut Self, lines: Lines) -> String {
        let g = self.parse_graph(lines);

        self.init(&g);

        let mut vec = vec![0; g.node_count()];

        self.count_path_2(self.start, false, &g, &mut vec)
            .to_string()
    }
}
