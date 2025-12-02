use std::{collections::HashSet, str::FromStr};

use anyhow::{Ok, bail};

use crate::aoc::DaySolver;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        input
            .iter()
            .flat_map(|line| line.split(','))
            .map(|string| string.parse::<IdRange>())
            .try_fold(0, |acc, range| {
                let sum: i64 = range?.get_invalid_ids().iter().sum();
                Ok(acc + sum)
            })
    }

    fn solve_part2(&self, input: &Vec<String>) -> anyhow::Result<i64> {
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
    let numbers = 0..amount;
    numbers.into_iter().fold(0, |acc, _| {
        acc * 10_i64.pow(number.checked_ilog10().unwrap_or(0) + 1) + number
    })
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

        let range = IdRange {
            lower: split
                .next()
                .expect("input malformated")
                .parse()
                .expect("Input not a number"),
            upper: split
                .next()
                .expect("input malformated")
                .parse()
                .expect("Input not a number"),
        };
        match split.next() {
            Some(_) => bail!("Input split too long"),
            None => Ok(range),
        }
    }
}

impl IdRange {
    fn in_range(self, value: i64) -> bool {
        self.lower <= value && self.upper >= value
    }

    fn get_invalid_ids(self) -> Vec<i64> {
        let half_length = (self.upper.checked_ilog10().unwrap_or(0) + 1).div_ceil(2);
        let first_halth =
            &self.upper.to_string()[..half_length.try_into().expect("Failed converting to usize")];

        let first_halth: i64 = first_halth.parse().expect("Failed to re-parse number");
        let candidates = 0_i64..=first_halth;

        candidates
            .into_iter()
            .map(|value| concat(value, 2))
            .filter(|id| self.in_range(*id))
            .collect()
    }

    fn get_invalid_ids_extended(self) -> HashSet<i64> {
        let full_length = self.upper.checked_ilog10().unwrap_or(0) + 1;
        let half_length = full_length.div_ceil(2);
        let lower_length = self.lower.checked_ilog10().unwrap_or(0) + 1;
        let first_halth =
            &self.upper.to_string()[..half_length.try_into().expect("Failed converting to usize")];

        let first_halth: i64 = first_halth.parse().expect("Failed to re-parse number");
        let candidates = 0_i64..=first_halth;

        candidates
            .into_iter()
            .flat_map(|value| {
                let mut set: HashSet<i64> = HashSet::new();
                let value_length = value.checked_ilog10().unwrap_or(0) + 1;
                let possible_range = lower_length..=full_length;
                possible_range.into_iter().for_each(|length| {
                    if length % value_length == 0 {
                        let id = concat(value, length / value_length);
                        if self.in_range(id) {
                            set.insert(id);
                        }
                    }
                });
                set
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

    #[test]
    fn test_solve_part1() {
        let input = vec![
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let day: Day = 2.try_into().unwrap();
        assert_eq!(1227775554, day.solve_part1(&input).unwrap());
    }

    #[test]
    fn test_solve_part2() {
        let input = vec![
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let day: Day = 2.try_into().unwrap();
        assert_eq!(4174379265, day.solve_part2(&input).unwrap());
    }
}
