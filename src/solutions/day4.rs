use std::ops::{Deref, DerefMut};

use anyhow::{Ok, bail};

use crate::aoc::DaySolver;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        Ok(Grid::new(input)?.count_accessible())
    }

    fn solve_part2(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        Ok(Grid::new(input)?.count_removable())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(i8)]
enum Coordinate {
    Roll = 1,
    Free = 0,
}

impl Coordinate {
    fn parse<S: AsRef<str>>(input: S) -> anyhow::Result<Vec<Coordinate>> {
        input.as_ref().chars().map(|c| c.try_into()).collect()
    }
}

impl TryFrom<char> for Coordinate {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Coordinate::Free,
            '@' => Coordinate::Roll,
            _ => bail!("Unparseable char"),
        })
    }
}

#[derive(Debug, PartialEq)]
struct Grid(Vec<Vec<Coordinate>>);

impl Grid {
    fn new<S: AsRef<str>>(input: &[S]) -> anyhow::Result<Grid> {
        Ok(Grid(
            input
                .iter()
                .map(Coordinate::parse)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }

    fn position_roll_and_accessible(&self, row: usize, col: usize) -> bool {
        let coordinate = self[row][col];
        match coordinate {
            Coordinate::Roll => {
                let mut neighbors = 0;
                let left = if row > 0 { row - 1 } else { 0 };
                let above = if col > 0 { col - 1 } else { 0 };

                (left..row + 2).for_each(|row_idx| {
                    (above..col + 2).for_each(|col_idx| {
                        if let Some(Coordinate::Roll) =
                            self.get(row_idx).and_then(|col| col.get(col_idx))
                        {
                            neighbors += 1;
                        }
                    })
                });
                neighbors < 5
            }
            Coordinate::Free => false,
        }
    }

    fn count_accessible(self) -> i64 {
        self.iter().enumerate().fold(0, |acc, (row_index, value)| {
            acc + value.iter().enumerate().fold(0, |acc, (col_index, _)| {
                if self.position_roll_and_accessible(row_index, col_index) {
                    acc + 1
                } else {
                    acc
                }
            })
        })
    }

    fn count_removable(&mut self) -> i64 {
        let mut removed = 0i64;
        loop {
            let last = removed;
            (0..self.len()).for_each(|row| {
                (0..self[row].len()).for_each(|col| {
                    if self.position_roll_and_accessible(row, col) {
                        self[row][col] = Coordinate::Free;
                        removed += 1;
                    }
                })
            });

            if last == removed {
                return removed;
            }
        }
    }
}

impl Deref for Grid {
    type Target = Vec<Vec<Coordinate>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::Day;

    #[test]
    fn test_solve_part1() {
        let input = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let day: Day = 4.try_into().unwrap();
        assert_eq!(13, day.solve_part1(&input).unwrap());
    }

    #[test]
    fn test_solve_part2() {
        let input = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let day: Day = 4.try_into().unwrap();
        assert_eq!(43, day.solve_part2(&input).unwrap());
    }
}
