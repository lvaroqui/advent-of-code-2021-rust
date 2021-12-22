use crate::Lines;

pub struct Solver {}

fn get_matching(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("Unexpected char received {}", c),
    }
}

impl crate::Solver for Solver {
    fn solve_part1(self: &mut Self, lines: Lines) -> String {
        let get_points = |c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Unexpected char received {}", c),
        };

        let mut score = 0;

        for line in lines {
            let mut stack = Vec::new();

            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => {
                        stack.push(c);
                    }
                    ')' | ']' | '}' | '>' => {
                        if stack.len() == 0 || *stack.last().unwrap() != get_matching(c) {
                            score += get_points(c);
                            break;
                        }
                        stack.pop();
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        score.to_string()
    }

    fn solve_part2(self: &mut Self, lines: Lines) -> String {
        let get_points = |c| match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Unexpected char received {}", c),
        };

        let mut scores: Vec<u64> = Vec::new();

        for line in lines {
            let mut stack = Vec::new();

            let mut skip = false;

            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => {
                        stack.push(c);
                    }
                    ')' | ']' | '}' | '>' => {
                        if stack.len() == 0 || *stack.last().unwrap() != get_matching(c) {
                            skip = true;
                            break;
                        }
                        stack.pop();
                    }
                    _ => {
                        continue;
                    }
                }
            }

            if skip {
                continue;
            }

            scores.push(stack.iter().rev().fold(0, |mut acc, val| {
                acc *= 5;
                acc += get_points(*val);
                acc
            }));
        }

        scores.sort();
        scores[scores.len() / 2].to_string()
    }
}
