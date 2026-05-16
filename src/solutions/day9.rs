use anyhow::Context;

use crate::aoc::DaySolver;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &[String]) -> anyhow::Result<i64> {
        let points = parse(input)?;
        Ok(points
            .iter()
            .enumerate()
            .flat_map(|(i, &(x1, y1))| {
                points[i + 1..].iter().map(move |&(x2, y2)| {
                    ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1)
                })
            })
            .max()
            .unwrap_or(0))
    }
}

fn parse(input: &[String]) -> anyhow::Result<Vec<(i64, i64)>> {
    input
        .iter()
        .map(|line| {
            let (x, y) = line.split_once(',').context("missing comma")?;
            Ok((x.trim().parse()?, y.trim().parse()?))
        })
        .collect()
}

#[cfg(test)]
mod tests {

    fn test_input() -> Vec<String> {
        vec![
            "7,1",
            "11,1",
            "11,7",
            "9,7",
            "9,5",
            "2,5",
            "2,3",
            "7,3",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    #[test]
    fn test_solve_part1() {
        let day: crate::aoc::Day = 9.try_into().unwrap();
        assert_eq!(50, day.solve_part1(&test_input()).unwrap());
    }

    #[test]
    fn test_solve_part2() {
        assert!(true)
    }
}
