use advent_of_code_2021::*;
use anyhow::bail;

use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <day> <part>", args[0]);
        bail!("Bad input parameters");
    }

    let day: u32 = args[1].parse().expect("Please provide a day number");
    let part: u32 = args[2].parse().expect("Please provide a part number");

    assert!(part == 1 || part == 2, "Invalid part number");

    // Retrieve inputs if they do not exists
    let input_dir = std::path::Path::new("inputs");
    if !input_dir.exists() {
        std::fs::create_dir(input_dir)?;
    }

    let input_path = input_dir.join(day.to_string());
    if !input_path.exists() {
        println!("Input does not exists yet, fetching it from Advent of Code website...");

        // Retrieve session key from cached file or user
        let session_path = input_dir.join("session");
        let mut session = String::new();
        if !session_path.exists() {
            println!("Please provide your session key:");
            io::stdin().read_line(&mut session)?;
            let mut file = std::fs::File::create(&session_path)?;
            file.write_all(session.as_bytes())?;
        } else {
            let mut file = std::fs::File::open(&session_path)?;
            file.read_to_string(&mut session).unwrap();
        }
        let session = session.trim_end();

        let client = reqwest::blocking::Client::builder().build()?;
        let res = client
            .get(format!("https://adventofcode.com/2021/day/{}/input", day))
            .header("Cookie", format!("session={}", session).as_str())
            .send()?;

        if res.status() == 500 {
            std::fs::remove_file(&session_path).unwrap();
            bail!("Cookie seems to be invalid");
        }

        let mut file = std::fs::File::create(&input_path)?;
        file.write_all(res.text().unwrap().as_bytes())?;
    }

    let file = fs::File::open(input_path).unwrap();

    let mut solver = get_solver(day);

    let lines = Lines::new(io::BufReader::new(file).lines());

    let res;
    if part == 1 {
        res = solver.solve_part1(lines);
    } else {
        res = solver.solve_part2(lines);
    }

    println!("Result for Day {} Part {} is {}", day, part, res);

    Ok(())
}
