use std::collections::BTreeMap;

use crate::Lines;

pub struct Solver {}

impl Solver {
    fn solve<F>(self: &mut Self, mut lines: Lines, cost_function: F) -> String
    where
        F: Fn(i64) -> i64,
    {
        let mut crabs = BTreeMap::<i64, i64>::new();
        let mut total = 0;
        lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .for_each(|v| {
                total += 1;
                *crabs.entry(v).or_insert(0) += 1
            });

        let mut min_cost = None;
        let min_pos = *crabs.iter().next().unwrap().0;
        let max_pos = *crabs.iter().next_back().unwrap().0;

        for pos in min_pos..=max_pos {
            let cost = crabs
                .iter()
                .fold(0, |c, (&p, &n)| c + cost_function((p - pos).abs()) * n);

            match min_cost {
                Some((_, min)) if cost < min => min_cost = Some((pos, cost)),
                None => min_cost = Some((pos, cost)),
                _ => (),
            }
        }

        min_cost.unwrap().1.to_string()
    }
}

impl crate::Solver for Solver {
    fn solve_part1(self: &mut Self, lines: Lines) -> String {
        self.solve(lines, |dist| dist)
    }

    fn solve_part2(self: &mut Self, lines: Lines) -> String {
        // Cost is the sum of the nth first natural number which can be computed as
        // (n(n+1)) / 2
        self.solve(lines, |dist| ((dist) * (dist + 1)) / 2)
    }
}
