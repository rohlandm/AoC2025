mod aoc;

use crate::aoc::daysolver::*;
use std::env;

pub fn main() {
    let day: u8 = env::args().find_map(|arg| arg.parse().ok()).unwrap();
    println!("🎄 Selected day: {day}");

    let solver: Box<dyn DaySolver> = match day {
        _ => Box::new(DefaultDaySolver),
    };

    match solver.solve_part1() {
        Ok(i) => println!("🎅🏼 The result of part 1 is {i}."),
        Err(_) => println!("😱 Part 1 is not yet implemented!"),
    }

    match solver.solve_part2() {
        Ok(i) => println!("🎅🏼 The result of part 2 is {i}."),
        Err(_) => println!("😱 Part 2 is not yet implemented!"),
    }
}
