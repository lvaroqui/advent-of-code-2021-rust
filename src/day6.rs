use crate::Lines;

pub struct Solver {}

impl Solver {
    fn solve(self: &mut Self, mut lines: Lines, days: u64) -> String {
        let mut fishes = [0u64; 9];
        lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .for_each(|v| fishes[v] += 1);

        for _ in 0..days {
            let reproducting = fishes[0];

            for i in 0..8 {
                fishes[i] = fishes[i + 1];
            }

            // Account for reproducting fishes that will go back to 7 days pregnancy
            fishes[6] += reproducting;

            // Babies!!
            fishes[8] = reproducting;
        }

        fishes.iter().sum::<u64>().to_string()
    }
}

impl crate::Solver for Solver {
    fn solve_part1(self: &mut Self, lines: Lines) -> String {
        self.solve(lines, 80)
    }

    fn solve_part2(self: &mut Self, lines: Lines) -> String {
        self.solve(lines, 256)
    }
}
