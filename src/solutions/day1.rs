use anyhow::bail;

use crate::aoc::DaySolver;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        let mut state: i32 = 50;

        input.iter().map(parse).try_fold(0, |acc, instruction| {
            let instruction = instruction?;
            state = (state + instruction).rem_euclid(100);
            if state == 0 {
                return Ok(acc + 1);
            }
            Ok(acc)
        })
    }

    fn solve_part2(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        let mut state = 50;

        input.iter().map(parse).try_fold(0i64, |acc, instruction| {
            let instruction = instruction?;
            let old_state = state;
            let acc = acc + (instruction.abs() / 100) as i64;
            state = (state + instruction).rem_euclid(100);

            if state == 0 || (old_state != 0 && (state - old_state) * instruction <= 0) {
                return Ok(acc + 1);
            }

            Ok(acc)
        })
    }
}

#[expect(clippy::ptr_arg, reason = "Else map fails with type mismatch")]
fn parse(line: &String) -> anyhow::Result<i32> {
    let (direction, value) = line.split_at(1);
    let value: i32 = value.parse()?;
    if value <= 0 {
        bail!("only positive rotation allowed");
    }

    match direction {
        "L" => Ok(-value),
        "R" => Ok(value),
        _ => bail!("unparseable input"),
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
