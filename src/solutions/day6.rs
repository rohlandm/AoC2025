use std::ops::Deref;

use anyhow::{Ok, bail, ensure};

use crate::aoc::DaySolver;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        let problems = parse_input(input)?;
        Ok(problems.iter().map(|problem| problem.solve()).sum())
    }

    fn solve_part2(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        let problems = parse_input_r2l(input)?;
        Ok(problems.iter().map(|problem| problem.solve()).sum())
    }
}

#[derive(Debug, Clone)]
enum Problem {
    Add(Vec<i64>),
    Mul(Vec<i64>),
}

impl Problem {
    fn solve(&self) -> i64 {
        match self {
            Problem::Add(items) => items.iter().sum(),
            Problem::Mul(items) => items.iter().product(),
        }
    }
}

impl TryFrom<&Vec<&str>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: &Vec<&str>) -> Result<Self, Self::Error> {
        match value.split_last() {
            Some((op, rest)) => {
                let numbers = rest
                    .iter()
                    .map(|s| s.parse::<i64>())
                    .collect::<Result<Vec<_>, _>>()?;
                match *op {
                    "+" => Ok(Problem::Add(numbers)),
                    "*" => Ok(Problem::Mul(numbers)),
                    _ => bail!("Unknown operator"),
                }
            }
            None => bail!("Empty Vector"),
        }
    }
}

fn parse_input(input: &[String]) -> anyhow::Result<Problems> {
    let transposed = transpose(
        input
            .iter()
            .map(|x| x.split_whitespace().collect())
            .collect(),
        "",
    )?;

    transposed.iter().map(Problem::try_from).collect()
}

fn parse_input_r2l(input: &[String]) -> anyhow::Result<Problems> {
    let (op_row, data_rows) = input.split_last().ok_or_else(|| anyhow::anyhow!("Empty input"))?;

    let char_cols = transpose(data_rows.iter().map(|s| s.chars().collect()).collect(), ' ')?;

    let cols: Vec<String> = char_cols
        .into_iter()
        .map(|v| v.into_iter().filter(|c| !c.is_whitespace()).collect())
        .collect();

    cols.split(|s: &String| s.is_empty())
        .zip(op_row.split_whitespace())
        .map(|(numbers, op)| {
            let nums = numbers
                .iter()
                .map(|s| s.parse::<i64>())
                .collect::<Result<Vec<_>, _>>()?;
            match op {
                "+" => Ok(Problem::Add(nums)),
                "*" => Ok(Problem::Mul(nums)),
                _ => bail!("Unknown operator {op}"),
            }
        })
        .collect()
}

fn transpose<T>(matrix: Vec<Vec<T>>, default: T) -> anyhow::Result<Vec<Vec<T>>>
where
    T: Clone,
{
    ensure!(!matrix.is_empty());
    let max_len = matrix.iter().map(|r| r.len()).max().unwrap_or(0);
    Ok((0..max_len)
        .map(|i| {
            matrix
                .iter()
                .map(|j| j.get(i).unwrap_or(&default).clone())
                .collect::<Vec<T>>()
        })
        .collect())
}

#[derive(Debug, Clone)]
struct Problems(Vec<Problem>);

impl FromIterator<Problem> for Problems {
    fn from_iter<T: IntoIterator<Item = Problem>>(iter: T) -> Self {
        Problems(iter.into_iter().collect())
    }
}

impl Deref for Problems {
    type Target = Vec<Problem>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::Day;

    #[test]
    fn test_solve_part1() {
        let input = vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let day: Day = 6.try_into().unwrap();
        assert_eq!(4277556, day.solve_part1(&input).unwrap());
    }

    #[test]
    fn test_solve_part2() {
        let input = vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let day: Day = 6.try_into().unwrap();
        assert_eq!(3263827, day.solve_part2(&input).unwrap());
    }
}
