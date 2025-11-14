mod aoc;

use crate::aoc::{
    day1solver::Day1Solver, day2solver::Day2Solver, day3solver::Day3Solver, day4solver::Day4Solver,
    day5solver::Day5Solver, day6solver::Day6Solver, day7solver::Day7Solver, day8solver::Day8Solver,
    day9solver::Day9Solver, day10solver::Day10Solver, day11solver::Day11Solver,
    day12solver::Day12Solver, daysolver::*, filereader,
};
use std::env;

pub fn main() {
    let day: u8 = match env::args().find_map(|arg| arg.parse().ok()) {
        Some(i) => i,
        None => {
            eprintln!("🚨 Missing or malformed day argument");
            return;
        }
    };

    match day {
        1 => run(Day1Solver),
        2 => run(Day2Solver),
        3 => run(Day3Solver),
        4 => run(Day4Solver),
        5 => run(Day5Solver),
        6 => run(Day6Solver),
        7 => run(Day7Solver),
        8 => run(Day8Solver),
        9 => run(Day9Solver),
        10 => run(Day10Solver),
        11 => run(Day11Solver),
        12 => run(Day12Solver),
        _ => {
            eprintln!("🚨 Day argument should be between 1 and 12");
        }
    }
}

fn run(solver: impl DaySolver) {
    let input = match filereader::read_file(&format!("input/{}.txt", solver.day())) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("🚨 Error reading input: {}", e);
            return;
        }
    };

    println!("🎄 Selected day: {}", solver.day());
    match solver.solve_part1(&input) {
        Ok(i) => println!("🛷 The result of part 1 is {i}."),
        Err(e) => println!("{}", e),
    }

    match solver.solve_part2(&input) {
        Ok(i) => println!("🛷 The result of part 2 is {i}."),
        Err(e) => println!("{}", e),
    }
}
