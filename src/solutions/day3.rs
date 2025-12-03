use std::str::FromStr;

use crate::aoc::DaySolver;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        Ok(input
            .iter()
            .flat_map(|line| line.parse::<BatteryBank>())
            .map(|bank| bank.calculate_joltage(2))
            .sum())
    }

    fn solve_part2(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        Ok(input
            .iter()
            .flat_map(|line| line.parse::<BatteryBank>())
            .map(|bank| bank.calculate_joltage(12))
            .sum())
    }
}

#[derive(Debug, Clone)]
struct BatteryBank {
    batteries: Vec<u32>,
}

impl BatteryBank {
    fn get_next_index(&self, start: usize, end: usize) -> usize {
        self.batteries[start..self.batteries.len() - end]
            .iter()
            .enumerate()
            .fold(0, |acc, (index, battery)| {
                if *battery > self.batteries[acc + start] {
                    index
                } else {
                    acc
                }
            })
            + start
    }

    fn calculate_joltage(&self, amount: u32) -> i64 {
        (0..amount)
            .rfold((0i64, 0), |(acc, start), rest| {
                let first = self.get_next_index(start, rest as usize);
                (acc * 10 + self.batteries[first] as i64, first + 1)
            })
            .0
    }
}

impl FromStr for BatteryBank {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BatteryBank {
            batteries: s.chars().flat_map(|c| c.to_digit(10)).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::Day;

    #[test]
    fn test_solve_part1() {
        let input = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let day: Day = 3.try_into().unwrap();
        assert_eq!(357, day.solve_part1(&input).unwrap());
    }

    #[test]
    fn test_solve_part2() {
        let input = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let day: Day = 3.try_into().unwrap();
        assert_eq!(3121910778619, day.solve_part2(&input).unwrap());
    }
}
