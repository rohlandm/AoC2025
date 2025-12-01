use std::{
    ops::{Add, Div, Mul},
    str::FromStr,
};

use anyhow::bail;

use crate::aoc::DaySolver;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        let mut state: i32 = 50;

        input
            .iter()
            .map(|line| line.parse::<Instruction>())
            .try_fold(0, |acc, instruction| {
                state = (state + instruction?).rem_euclid(100);
                if state == 0 {
                    return Ok(acc + 1);
                }
                Ok(acc)
            })
    }

    fn solve_part2(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        let mut state = 50;

        input
            .iter()
            .map(|line| line.parse::<Instruction>())
            .try_fold(0i64, |acc, instruction| {
                let instruction = instruction?;
                let old_state = state;
                let acc = acc + (instruction / 100) as i64;
                state = (state + instruction).rem_euclid(100);

                if state == 0 || (old_state != 0 && (state - old_state) * instruction <= 0) {
                    return Ok(acc + 1);
                }

                Ok(acc)
            })
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Left(i32),
    Right(i32),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (direction, value) = line.split_at(1);
        match direction {
            "L" => Ok(Instruction::Left(value.parse()?)),
            "R" => Ok(Instruction::Right(value.parse()?)),
            _ => bail!("unparseable input"),
        }
    }
}

impl Add<Instruction> for i32 {
    fn add(self, rhs: Instruction) -> Self::Output {
        match rhs {
            Instruction::Left(x) => self - x,
            Instruction::Right(x) => self + x,
        }
    }

    type Output = Self;
}

impl Div<i32> for Instruction {
    type Output = i32;

    fn div(self, rhs: i32) -> Self::Output {
        let lhs = match self {
            Instruction::Left(x) => x,
            Instruction::Right(x) => x,
        };
        lhs / rhs
    }
}

impl Mul<Instruction> for i32 {
    type Output = i32;

    fn mul(self, rhs: Instruction) -> Self::Output {
        let rhs = match rhs {
            Instruction::Left(x) => -x,
            Instruction::Right(x) => x,
        };
        self * rhs
    }
}
#[cfg(test)]
mod tests {
    use crate::aoc::Day;

    #[test]
    fn test_solve_part1() {
        let input = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let day: Day = 1.try_into().unwrap();
        assert_eq!(3, day.solve_part1(&input).unwrap());
    }

    #[test]
    fn test_solve_part2() {
        let input = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let day: Day = 1.try_into().unwrap();
        assert_eq!(6, day.solve_part2(&input).unwrap());

        let input = vec![
            "L68", "L30", "R148", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let day: Day = 1.try_into().unwrap();
        assert_eq!(7, day.solve_part2(&input).unwrap());
    }
}
