use anyhow::bail;

use crate::aoc::DaySolver;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, _input: &Vec<String>) -> anyhow::Result<i64> {
        let instructions: anyhow::Result<Vec<Instruction>> =
            _input.iter().map(|line| parse(line)).collect();
        let instructions = instructions?;

        let mut state = 50;
        let password = instructions.iter().fold(0, |acc, instruction| {
            match instruction {
                Instruction::Left(amount) => state = (state - amount).rem_euclid(100),
                Instruction::Right(amount) => state = (state + amount).rem_euclid(100),
            };
            if state == 0 {
                return acc + 1;
            }
            acc
        });

        Ok(password)
    }

    fn solve_part2(&self, _input: &Vec<String>) -> anyhow::Result<i64> {
        let instructions: anyhow::Result<Vec<Instruction>> =
            _input.iter().map(|line| parse(line)).collect();
        let instructions = instructions?;

        let mut state = 50;
        let password = instructions
            .iter()
            .fold(0, |acc, instruction| match instruction {
                Instruction::Left(amount) => {
                    let old_state = state;
                    let rotations = amount / 100;
                    state = (state - amount).rem_euclid(100);
                    if state == 0 || (old_state != 0 && state >= old_state) {
                        return acc + 1 + rotations;
                    }

                    acc + rotations
                }
                Instruction::Right(amount) => {
                    let old_state = state;
                    let rotations = amount / 100;
                    state = (state + amount).rem_euclid(100);
                    if state == 0 || (old_state != 0 && state <= old_state) {
                        return acc + 1 + rotations;
                    }

                    acc + rotations
                }
            });

        Ok(password.into())
    }
}

fn parse(line: &str) -> anyhow::Result<Instruction> {
    let tuple = line.split_at(1);
    match tuple.0 {
        "L" => Ok(Instruction::Left(tuple.1.parse()?)),
        "R" => Ok(Instruction::Right(tuple.1.parse()?)),
        _ => bail!("unparseable input"),
    }
}

enum Instruction {
    Left(i32),
    Right(i32),
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
