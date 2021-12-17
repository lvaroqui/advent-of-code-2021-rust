use crate::Lines;

pub struct Solver1 {}

impl crate::Solver for Solver1 {
    fn solve(self: &mut Self, lines: Lines) -> String {
        let mut previous = None;
        let mut increased = 0;

        for line in lines {
            let val: u32 = line.parse().unwrap();

            match previous {
                Some(prev) if val > prev => increased += 1,
                _ => (),
            }

            previous = Some(val);
        }

        increased.to_string()
    }
}

pub struct Solver2 {}

impl crate::Solver for Solver2 {
    fn solve(self: &mut Self, mut lines: Lines) -> String {
        let mut previous: Option<u32> = None;
        let mut increased = 0;
        let mut window = [0; 3];

        let mut parse_line = || lines.next().unwrap().parse::<u32>().unwrap();
        window[0] = parse_line();
        window[1] = parse_line();
        window[2] = parse_line();

        for line in lines {
            window[0] = window[1];
            window[1] = window[2];
            window[2] = line.parse::<u32>().unwrap();

            let sum = window.iter().sum();

            match previous {
                Some(prev) if sum > prev => increased += 1,
                _ => (),
            }

            previous = Some(sum);
        }

        increased.to_string()
    }
}
