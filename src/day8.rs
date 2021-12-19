use std::collections::{BTreeSet, HashMap, HashSet};

use crate::Lines;

pub struct Solver {}

type Pattern = BTreeSet<char>;

impl crate::Solver for Solver {
    fn solve_part1(self: &mut Self, lines: Lines) -> String {
        let mut count = 0;
        for line in lines {
            line.split('|').nth(1).unwrap().split(' ').for_each(|s| {
                if let 2 | 4 | 3 | 7 = s.len() {
                    count += 1
                }
            })
        }
        count.to_string()
    }

    fn solve_part2(self: &mut Self, lines: Lines) -> String {
        let mut sum = 0;

        for line in lines {
            let mut line = line.split('|');
            let mut input_patterns = line
                .next()
                .unwrap()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().collect::<Pattern>())
                .collect::<HashSet<_>>();

            let mut patterns = HashMap::<u8, Pattern>::new();

            // Identify 1, 4, 7, 8 by number of bars
            input_patterns.retain(|p| {
                match p.len() {
                    2 => patterns.insert(1, p.clone()),
                    4 => patterns.insert(4, p.clone()),
                    3 => patterns.insert(7, p.clone()),
                    7 => patterns.insert(8, p.clone()),
                    _ => return true,
                };
                return false;
            });

            // Identify 6 since it is the only 6 bar pattern without a c, this
            // means there should be only one 6 bar pattern where all bars from
            // number 1 are not here, this missing bar is c, the other bar of 1
            // is by deduction f
            let one_bars = patterns[&1].iter().map(|s| *s).collect::<Vec<_>>();
            let mut c = None;
            let mut f = None;
            input_patterns.retain(|p| {
                if p.len() != 6 {
                    return true;
                }

                if !p.contains(&one_bars[0]) {
                    c = Some(one_bars[0]);
                    f = Some(one_bars[1]);
                } else if !p.contains(&one_bars[1]) {
                    c = Some(one_bars[1]);
                    f = Some(one_bars[0]);
                } else {
                    return true;
                }
                patterns.insert(6, p.clone());
                false
            });
            let c = c.unwrap();
            let f = f.unwrap();

            // Disambiguate 5 bar patterns (2, 3 and 5) using their difference on c and f
            // 2: only c
            // 3: c and f
            // 5: only f
            input_patterns.retain(|p| {
                if p.len() != 5 {
                    return true;
                }

                match (p.contains(&c), p.contains(&f)) {
                    (true, false) => patterns.insert(2, p.clone()),
                    (true, true) => patterns.insert(3, p.clone()),
                    (false, true) => patterns.insert(5, p.clone()),
                    _ => panic!("There should not be a 5 bar number without c and f {:?}", p),
                };

                false
            });

            // Disambiguate remaining patterns (9 and 0) using that their only
            // difference with number 4 is the d bar missing on 0
            let four = patterns[&4].clone();
            input_patterns.retain(|p| {
                for bar in four.iter() {
                    if !p.contains(&bar) {
                        patterns.insert(0, p.clone());
                        return false;
                    }
                }
                true
            });

            input_patterns.retain(|p| {
                patterns.insert(9, p.clone());
                false
            });
            assert_eq!(input_patterns.len(), 0);

            line.next()
                .unwrap()
                .split(' ')
                .rev() // Iterate from behind so that "i" matches the power of ten to apply
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().collect::<Pattern>())
                .enumerate()
                .for_each(|(i, pattern)| {
                    for (&value, p) in patterns.iter() {
                        if *p == pattern {
                            sum += value as u32 * 10_u32.pow(i as u32);
                        }
                    }
                });
        }
        sum.to_string()
    }
}
