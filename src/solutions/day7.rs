use std::collections::{HashMap, HashSet};

use crate::aoc::DaySolver;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &[String]) -> anyhow::Result<i64> {
        let grid = parse_grid(input);
        let s_col = find_s(&grid)?;
        Ok(count_splits(&grid, 0, s_col))
    }

    fn solve_part2(&self, input: &[String]) -> anyhow::Result<i64> {
        let grid = parse_grid(input);
        let s_col = find_s(&grid)?;
        Ok(count_timelines(&grid, 0, s_col))
    }
}

fn parse_grid(input: &[String]) -> Vec<Vec<char>> {
    input.iter().map(|line| line.chars().collect()).collect()
}

fn find_s(grid: &[Vec<char>]) -> anyhow::Result<usize> {
    grid[0]
        .iter()
        .position(|&c| c == 'S')
        .ok_or_else(|| anyhow::anyhow!("No S found in first row"))
}

fn next_splitter(grid: &[Vec<char>], row: usize, col: usize) -> Option<usize> {
    (row..grid.len()).find(|&r| grid[r][col] == '^')
}

fn count_timelines(grid: &[Vec<char>], start_row: usize, col: usize) -> i64 {
    let mut memo = HashMap::new();
    count_timelines_memo(grid, grid[0].len(), start_row, col, &mut memo)
}

fn count_timelines_memo(
    grid: &[Vec<char>],
    width: usize,
    start_row: usize,
    col: usize,
    memo: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    if let Some(&cached) = memo.get(&(start_row, col)) {
        return cached;
    }
    let result = next_splitter(grid, start_row, col).map_or(1, |splitter_row| {
        [col.checked_sub(1), Some(col + 1).filter(|&c| c < width)]
            .into_iter()
            .map(|nc| nc.map_or(1, |c| count_timelines_memo(grid, width, splitter_row, c, memo)))
            .sum()
    });
    memo.insert((start_row, col), result);
    result
}

fn count_splits(grid: &[Vec<char>], start_row: usize, start_col: usize) -> i64 {
    let width = grid[0].len();
    let mut visited = HashSet::new();
    let mut stack = vec![(start_row, start_col)];

    while let Some((row, col)) = stack.pop() {
        if let Some(splitter_row) = next_splitter(grid, row, col)
            && visited.insert((splitter_row, col)) {
                stack.extend(
                    [col.checked_sub(1), Some(col + 1).filter(|&c| c < width)]
                        .into_iter()
                        .flatten()
                        .map(|c| (splitter_row, c)),
                );
            }
    }

    visited.len() as i64
}

#[cfg(test)]
mod tests {
    use crate::aoc::Day;

    fn test_input() -> Vec<String> {
        vec![
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    #[test]
    fn test_solve_part1() {
        let day: Day = 7.try_into().unwrap();
        assert_eq!(21, day.solve_part1(&test_input()).unwrap());
    }

    #[test]
    fn test_solve_part2() {
        let day: Day = 7.try_into().unwrap();
        assert_eq!(40, day.solve_part2(&test_input()).unwrap());
    }
}
