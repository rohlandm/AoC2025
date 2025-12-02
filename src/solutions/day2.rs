use std::str::FromStr;

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
}

fn concat(numbers: &[i64]) -> i64 {
    numbers.iter().fold(0, |acc, elem| {
        acc * 10_i64.pow(elem.checked_ilog10().unwrap_or(0) + 1) + elem
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
            .map(|value| concat(&[value, value]))
            .filter(|id| self.in_range(*id))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{aoc::Day, solutions::day2::concat};

    #[test]
    fn test_concat() {
        assert_eq!(1234, concat(&[12, 34]));
        assert_eq!(1234, concat(&[123, 4]));
        assert_eq!(1234, concat(&[1, 2, 34]));
        assert_eq!(10000234, concat(&[100002, 34]));
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
        assert!(true)
    }
}
