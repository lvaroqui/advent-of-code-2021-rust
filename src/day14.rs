use std::{
    collections::{HashMap, LinkedList},
    str::FromStr,
};

use crate::Lines;

#[derive(Debug)]
struct Rule {
    from: [char; 2],
    to: char,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" -> ");

        let mut from = split.next().unwrap().chars();

        Ok(Rule {
            from: [from.next().unwrap(), from.next().unwrap()],
            to: split.next().unwrap().chars().next().unwrap(),
        })
    }
}

pub struct Solver {}

impl Solver {
    fn get_result(&self, letter_counts: &HashMap<char, u64>) -> u64 {
        let max = letter_counts.iter().map(|(_, v)| *v).max().unwrap();
        let min = letter_counts.iter().map(|(_, v)| *v).min().unwrap();

        max - min
    }
}

impl crate::Solver for Solver {
    // Naive implementation using simulation of polymer with linked list
    // Does not scale due to exponential complexity
    fn solve_part1(self: &mut Self, mut lines: Lines) -> String {
        // Parse initial polymer
        let mut polymer = lines.next().unwrap().chars().collect::<LinkedList<_>>();

        // Skip blank line
        lines.next().unwrap();

        // Parse rules
        let rules = lines
            .into_iter()
            .map(|l| Rule::from_str(&l).unwrap())
            .collect::<Vec<_>>();

        // Polymerisation!!
        for _ in 0..10 {
            let mut new_polymer = LinkedList::new();
            for (a, b) in polymer
                .iter()
                .zip(polymer.iter().skip(1).chain(std::iter::once(&'_')))
            {
                new_polymer.push_back(*a);
                let new_char = rules.iter().find_map(|rule| {
                    if rule.from[0] == *a && rule.from[1] == *b {
                        Some(rule.to)
                    } else {
                        None
                    }
                });

                if let Some(c) = new_char {
                    new_polymer.push_back(c);
                }
            }

            polymer = new_polymer;
        }

        let mut letter_counts = HashMap::new();

        for letter in polymer {
            *letter_counts.entry(letter).or_insert(0) += 1u64;
        }

        self.get_result(&letter_counts).to_string()
    }

    // High performance implementantion using the fact that we only care about
    // pairs and not their exact position. We compute effect for unique pairs
    // all at the same time.
    fn solve_part2(self: &mut Self, mut lines: Lines) -> String {
        // Parse initial polymer
        let start_polymer = lines.next().unwrap().chars().collect::<Vec<_>>();

        // Skip blank line
        lines.next().unwrap();

        // Parse rules
        let rules = lines
            .into_iter()
            .map(|l| Rule::from_str(&l).unwrap())
            .collect::<Vec<_>>();

        let mut letter_counts = HashMap::new();

        let mut pairs = HashMap::new();

        for (a, b) in start_polymer
            .iter()
            .zip(start_polymer.iter().skip(1).chain(std::iter::once(&'_')))
        {
            let e = letter_counts.entry(*a).or_insert(0);
            *e += 1u64;

            let e = pairs.entry([*a, *b]).or_insert(0);
            *e += 1u64;
        }

        // Polymerize!!
        for _ in 0..40 {
            let mut new_pairs = HashMap::new();
            for (pair, count) in pairs {
                if let Some(Rule { from: _, to: new }) = rules.iter().find(|r| r.from == pair) {
                    // Register newly created pairs
                    *new_pairs.entry([pair[0], *new]).or_insert(0) += count;
                    *new_pairs.entry([*new, pair[1]]).or_insert(0) += count;

                    // Count new character
                    *letter_counts.entry(*new).or_insert(0) += count;
                } else {
                    // No rule, simply pass the pair as is
                    *new_pairs.entry(pair).or_insert(0) += count;
                }
            }

            pairs = new_pairs;
        }

        self.get_result(&letter_counts).to_string()
    }
}
