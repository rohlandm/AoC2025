use std::str::FromStr;

use crate::aoc::DaySolver;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        Ok(input
            .iter()
            .flat_map(|line| line.parse::<BatteryBank>())
            .map(|bank| bank.calculate_joltage())
            .map(i64::from)
            .sum())
    }
}

#[derive(Debug, Clone)]
struct BatteryBank {
    batteries: Vec<u32>,
}

impl BatteryBank {
    fn calculate_joltage(self) -> u32 {
        let index_of_max = self
            .batteries
            .iter()
            .enumerate()
            .fold(0, |acc, (index, battery)| {
                if *battery > self.batteries[acc] {
                    index
                } else {
                    acc
                }
            });

        let index_of_max = if index_of_max == self.batteries.len() - 1 {
            self.batteries[..index_of_max]
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(index, _)| index)
                .expect("Bank to small")
        } else {
            index_of_max
        };

        let first = self
            .batteries
            .get(index_of_max)
            .expect("calculated index out of bound");

        let remainder = &self.batteries[index_of_max + 1..];

        let index_of_second_max = remainder
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .expect("Bank to small");

        let second = remainder
            .get(index_of_second_max)
            .expect("calculated index out of bound");

        first * 10u32.pow(second.ilog10() + 1) + second
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
            "333333333333331",
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let day: Day = 3.try_into().unwrap();
        assert_eq!(390, day.solve_part1(&input).unwrap());
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
