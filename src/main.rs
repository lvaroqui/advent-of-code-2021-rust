use advent_of_code_2021::*;

use std::env;
use std::fs;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <day> <part>", args[0]);
        return;
    }

    let day: u32 = args[1].parse().expect("Please provide a day number");
    let part: u32 = args[2].parse().expect("Please provide a part number");

    assert!(part == 1 || part == 2, "Invalid part number");

    let input_path = format!("inputs/{}", day);

    let file = fs::File::open(input_path).expect("Something went wrong reading the file");

    let mut solver = get_solver(day);

    let lines = Lines::new(io::BufReader::new(file).lines());

    let res;
    if part == 1 {
        res = solver.solve_part1(lines);
    } else {
        res = solver.solve_part2(lines);
    }

    println!("Result for Day {} Part {} is {}", day, part, res);
}
