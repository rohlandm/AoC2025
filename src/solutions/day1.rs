use std::str::FromStr;

use anyhow::bail;

use crate::aoc::DaySolver;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &[String]) -> anyhow::Result<i64> {
        input
            .iter()
            .map(|line| line.parse::<Instruction>())
            .try_fold((0i64, 50i32), |(acc, state), instruction| {
                let state = (state + instruction?.delta()).rem_euclid(100);
                Ok((acc + (state == 0) as i64, state))
            })
            .map(|(acc, _)| acc)
    }

    fn solve_part2(&self, input: &[String]) -> anyhow::Result<i64> {
        input
            .iter()
            .map(|line| line.parse::<Instruction>())
            .try_fold((0i64, 50i32), |(acc, state), instruction| {
                let delta = instruction?.delta();
                let new_state = (state + delta).rem_euclid(100);
                let acc = acc + (delta.abs() / 100) as i64;
                let hit = new_state == 0 || (state != 0 && (new_state - state) * delta <= 0);
                Ok((acc + hit as i64, new_state))
            })
            .map(|(acc, _)| acc)
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Left(i32),
    Right(i32),
}

impl Instruction {
    fn delta(self) -> i32 {
        match self {
            Instruction::Left(x) => -x,
            Instruction::Right(x) => x,
        }
    }
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
