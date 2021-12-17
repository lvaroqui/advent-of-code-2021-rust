use std::collections::HashSet;

use crate::Lines;

pub struct Solver {}

impl crate::Solver for Solver {
    fn solve_part1(self: &mut Self, lines: Lines) -> String {
        let mut total = 0;
        let mut ones = [0; 12];

        for line in lines {
            for (i, c) in line.chars().enumerate() {
                if c == '1' {
                    ones[i] += 1;
                }
            }
            total += 1;
        }

        // Place one in gamma where we have more ones than zeros
        let gamma = ones.iter().enumerate().fold(0, |mut num, (i, &val)| {
            if val > total / 2 {
                num |= 1 << (11 - i);
            }
            num
        });

        let epsilon = (!gamma) & (0b1111_1111_1111);

        (epsilon * gamma).to_string()
    }

    fn solve_part2(self: &mut Self, lines: Lines) -> String {
        let lines = lines.collect::<HashSet<_>>();

        let find_by_criteria =
            |mut working_set: HashSet<String>, keep: Box<dyn Fn(usize, usize) -> char>| {
                for i in 0..12 {
                    let mut ones = 0;
                    for line in working_set.iter() {
                        if line.chars().nth(i).unwrap() == '1' {
                            ones += 1;
                        }
                    }
                    let zeros = working_set.len() - ones;

                    working_set.retain(|v| v.chars().nth(i).unwrap() == keep(ones, zeros));
                    if working_set.len() == 1 {
                        return u32::from_str_radix(working_set.iter().next().unwrap(), 2).unwrap();
                    }
                }
                0
            };

        let life_support_rating = find_by_criteria(
            lines.clone(),
            Box::new(|ones, zeros| if ones >= zeros { '1' } else { '0' }),
        );

        let co2_scruber_rating = find_by_criteria(
            lines,
            Box::new(|ones, zeros| if ones >= zeros { '0' } else { '1' }),
        );

        (life_support_rating * co2_scruber_rating).to_string()
    }
}
