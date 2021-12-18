use crate::Lines;

pub struct Solver {}

type Board = [[(u32, bool); 5]; 5];

impl Solver {
    fn parse(mut lines: Lines) -> (Vec<u32>, Vec<Board>) {
        let numbers = lines
            .next()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let mut boards = Vec::<Board>::new();

        while let Some(_) = lines.next() {
            boards.push([[(0, false); 5]; 5]);
            let board = boards.last_mut().unwrap();
            for i in 0..5 {
                lines
                    .next()
                    .unwrap()
                    .split(' ')
                    .filter(|&x| !x.is_empty())
                    .enumerate()
                    .for_each(|(j, v)| board[i][j].0 = v.parse().unwrap());
            }
        }

        (numbers, boards)
    }

    fn set_number(board: &mut Board, value: u32) {
        board
            .iter_mut()
            .flatten()
            .filter(|(v, _)| *v == value)
            .for_each(|(_, b)| *b = true);
    }

    fn check_winner(board: &Board) -> bool {
        // Check for winning line
        for line in board.iter() {
            let mut ok = 0;
            for &(_, b) in line {
                if b {
                    ok += 1;
                }
            }
            if ok == 5 {
                return true;
            }
        }

        // Check for winning collumn
        for i in 0..5 {
            let mut ok = 0;
            for j in 0..5 {
                if board[j][i].1 {
                    ok += 1;
                }
            }
            if ok == 5 {
                return true;
            }
        }
        false
    }

    fn sum_of_unmarked(board: &Board) -> u32 {
        board
            .iter()
            .flatten()
            .filter_map(|&(v, b)| if !b { Some(v) } else { None })
            .sum()
    }
}

impl crate::Solver for Solver {
    fn solve_part1(self: &mut Self, lines: Lines) -> String {
        let (numbers, mut boards) = Solver::parse(lines);

        for i in numbers {
            for board in boards.iter_mut() {
                Solver::set_number(board, i);

                if Solver::check_winner(&board) {
                    return (i * Solver::sum_of_unmarked(&board)).to_string();
                }
            }
        }

        "".to_string()
    }

    fn solve_part2(self: &mut Self, lines: Lines) -> String {
        let (numbers, boards) = Solver::parse(lines);

        let mut boards = boards.iter().map(|b| (false, *b)).collect::<Vec<_>>();

        let mut win = 0;
        let total = boards.len();

        for i in numbers {
            for (winned, board) in boards.iter_mut() {
                if *winned {
                    continue;
                }

                Solver::set_number(board, i);

                if Solver::check_winner(&board) {
                    *winned = true;
                    win += 1;
                }

                if win == total {
                    return (i * Solver::sum_of_unmarked(&board)).to_string();
                }
            }
        }

        "".to_string()
    }
}
