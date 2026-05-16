use std::{collections::HashSet, str::FromStr};

use anyhow::{Ok, bail};

use crate::aoc::DaySolver;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &[String]) -> anyhow::Result<i64> {
        input
            .iter()
            .flat_map(|line| line.split(','))
            .map(|string| string.parse::<IdRange>())
            .try_fold(0, |acc, range| {
                let sum: i64 = range?.get_invalid_ids().iter().sum();
                Ok(acc + sum)
            })
    }

    fn solve_part2(&self, input: &[String]) -> anyhow::Result<i64> {
        input
            .iter()
            .flat_map(|line| line.split(','))
            .map(|string| string.parse::<IdRange>())
            .try_fold(0, |acc, range| {
                let sum: i64 = range?.get_invalid_ids_extended().iter().sum();
                Ok(acc + sum)
            })
    }
}

fn concat(number: i64, amount: u32) -> i64 {
    let digits = 10_i64.pow(number.checked_ilog10().unwrap_or(0) + 1);
    (0..amount).fold(0, |acc, _| acc * digits + number)
}

#[derive(Debug, Clone, Copy)]
struct IdRange {
    lower: i64,
    upper: i64,
}

impl FromStr for IdRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        let lower = split.next().ok_or_else(|| anyhow::anyhow!("input malformatted"))?.parse()?;
        let upper = split.next().ok_or_else(|| anyhow::anyhow!("input malformatted"))?.parse()?;
        if split.next().is_some() {
            bail!("Input split too long");
        }
        Ok(IdRange { lower, upper })
    }
}

impl IdRange {
    fn in_range(self, value: i64) -> bool {
        self.lower <= value && self.upper >= value
    }

    fn get_invalid_ids(self) -> Vec<i64> {
        let half_length = (self.upper.checked_ilog10().unwrap_or(0) + 1).div_ceil(2) as usize;
        let first_half: i64 = self.upper.to_string()[..half_length].parse().expect("Failed to re-parse number");

        (0_i64..=first_half)
            .map(|value| concat(value, 2))
            .filter(|id| self.in_range(*id))
            .collect()
    }

    fn get_invalid_ids_extended(self) -> HashSet<i64> {
        let full_length = self.upper.checked_ilog10().unwrap_or(0) + 1;
        let half_length = full_length.div_ceil(2) as usize;
        let lower_length = self.lower.checked_ilog10().unwrap_or(0) + 1;
        let first_half: i64 = self.upper.to_string()[..half_length].parse().expect("Failed to re-parse number");

        (0_i64..=first_half)
            .flat_map(|value| {
                let value_length = value.checked_ilog10().unwrap_or(0) + 1;
                (lower_length..=full_length)
                    .filter(move |length| length % value_length == 0)
                    .map(move |length| concat(value, length / value_length))
                    .filter(move |&id| self.in_range(id))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{aoc::Day, solutions::day2::concat};

    #[test]
    fn test_concat() {
        assert_eq!(12, concat(12, 1));
        assert_eq!(1212, concat(12, 2));
        assert_eq!(121212, concat(12, 3));
        assert_eq!(12121212, concat(12, 4));
    }

    fn test_input() -> Vec<String> {
        vec![
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    #[test]
    fn test_solve_part1() {
        let day: Day = 2.try_into().unwrap();
        assert_eq!(1227775554, day.solve_part1(&test_input()).unwrap());
    }

    #[test]
    fn test_solve_part2() {
        let day: Day = 2.try_into().unwrap();
        assert_eq!(4174379265, day.solve_part2(&test_input()).unwrap());
    }
}
