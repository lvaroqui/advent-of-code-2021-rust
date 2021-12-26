use crate::Lines;

use petgraph::graph::NodeIndex;
use petgraph::{Graph, Undirected};

pub struct Solver {}

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

    fn find_node_index(&self, g: &MyGraph, name: &str) -> NodeIndex {
        g.node_indices().find(|i| g[*i] == name).unwrap()
    }

    fn is_cave_small(&self, g: &MyGraph, index: NodeIndex) -> bool {
        g[index].chars().any(|c| c.is_lowercase())
    }

    fn count_path(
        &self,
        n: NodeIndex,
        end: NodeIndex,
        g: &MyGraph,
        visited: &mut Vec<bool>,
    ) -> u32 {
        let mut sum = 0;

        if n == end {
            return 1;
        }

        visited[n.index()] = self.is_cave_small(&g, n);

        for other in g.neighbors(n) {
            if visited[other.index()] {
                continue;
            }
            sum += self.count_path(other, end, &g, &mut visited.clone());
        }

        sum
    }

    fn count_path_2(
        &self,
        n: NodeIndex,
        start: NodeIndex,
        end: NodeIndex,
        g: &MyGraph,
        visited: &mut Vec<u32>,
    ) -> u32 {
        let mut sum = 0;

        if n == end {
            return 1;
        }

        if self.is_cave_small(&g, n) {
            visited[n.index()] += 1;
        }

        for other in g.neighbors(n) {
            if other == start {
                continue;
            }

            if self.is_cave_small(&g, other)
                && visited[other.index()] >= 1
                && visited.iter().any(|n| *n == 2)
            {
                continue;
            }

            sum += self.count_path_2(other, start, end, &g, &mut visited.clone());
        }

        sum
    }
}

impl crate::Solver for Solver {
    fn solve_part1(self: &mut Self, lines: Lines) -> String {
        let g = self.parse_graph(lines);

        let start = self.find_node_index(&g, "start");
        let end = self.find_node_index(&g, "end");

        let mut vec = vec![false; g.node_count()];
        vec[start.index()] = true;

        self.count_path(start, end, &g, &mut vec).to_string()
    }

    fn solve_part2(self: &mut Self, lines: Lines) -> String {
        let g = self.parse_graph(lines);

        let start = self.find_node_index(&g, "start");
        let end = self.find_node_index(&g, "end");

        let mut vec = vec![0; g.node_count()];

        self.count_path_2(start, start, end, &g, &mut vec)
            .to_string()
    }
}
