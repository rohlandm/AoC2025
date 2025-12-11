use std::str::FromStr;

use anyhow::Ok;

use crate::aoc::DaySolver;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        let split_index = input
            .iter()
            .position(|x| x.is_empty())
            .expect("Vector should contian an empty string");
        let mut ranges = input.to_vec();
        let values: Vec<i64> = ranges
            .split_off(split_index)
            .iter()
            .skip(1)
            .map(|x| x.parse::<i64>().expect("Values should be parseable"))
            .collect();

        let ranges: Vec<Range> = ranges
            .iter()
            .map(|x| x.parse::<Range>().expect("all ranges should be parseable"))
            .collect();
        Ok(values.iter().fold(0i64, |acc, value| {
            if ranges.iter().any(|range| range.contains_value(*value)) {
                acc + 1
            } else {
                acc
            }
        }))
    }
}

struct Range {
    min: i64,
    max: i64,
}

impl Range {
    fn contains_value(&self, value: i64) -> bool {
        self.min <= value && self.max >= value
    }
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("-");

        Ok(Range {
            min: split
                .next()
                .map(|x| x.parse())
                .expect("unparseable range")?,
            max: split
                .next()
                .map(|x| x.parse())
                .expect("unparseable range")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::Day;

    #[test]
    fn test_solve_part1() {
        let input = vec![
            "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let day: Day = 5.try_into().unwrap();
        assert_eq!(3, day.solve_part1(&input).unwrap());
    }

    #[test]
    fn test_solve_part2() {
        assert!(true)
    }
}
