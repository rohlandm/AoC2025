use crate::aoc::DaySolver;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &[String]) -> anyhow::Result<i64> {
        Ok(input.iter()
            .map(|line| {
                let (target, buttons) = parse_machine(line);
                min_presses(&target, &buttons)
            })
            .sum())
    }
}

fn parse_machine(line: &str) -> (Vec<bool>, Vec<Vec<usize>>) {
    let (bracket, rest) = line.split_once(']').expect("missing ]");
    let target = bracket.trim_start_matches('[').chars().map(|c| c == '#').collect();
    let buttons = rest
        .split('(')
        .skip(1)
        .filter_map(|s| {
            let btn: Vec<usize> = s.split(')').next()?
                .split(',')
                .filter_map(|n| n.trim().parse().ok())
                .collect();
            (!btn.is_empty()).then_some(btn)
        })
        .collect();
    (target, buttons)
}

fn min_presses(target: &[bool], buttons: &[Vec<usize>]) -> i64 {
    let n = target.len();
    let m = buttons.len();

    // Augmented GF(2) matrix: row i is the equation for light i.
    // Bit j = 1 iff button j toggles light i; bit m = target[i] (RHS).
    let mut rows: Vec<u128> = (0..n)
        .map(|i| {
            buttons.iter().enumerate()
                .filter(|(_, b)| b.contains(&i))
                .fold(u128::from(target[i]) << m, |acc, (j, _)| acc | (1 << j))
        })
        .collect();

    // Gaussian elimination over GF(2) → reduced row echelon form.
    // pivot_col[r] = the pivot column for row r; grows by one per pivot found.
    let mut pivot_col: Vec<usize> = Vec::new();

    for col in 0..m {
        let rank = pivot_col.len();
        if let Some(p) = (rank..n).find(|&r| rows[r] & (1 << col) != 0) {
            rows.swap(rank, p);
            let pivot = rows[rank];
            rows.iter_mut().enumerate().for_each(|(r, row)| {
                if r != rank && *row & (1 << col) != 0 {
                    *row ^= pivot;
                }
            });
            pivot_col.push(col);
        }
    }

    let free: Vec<usize> = (0..m).filter(|j| !pivot_col.contains(j)).collect();

    (0u128..(1 << free.len()))
        .map(|mask| {
            let mut x = vec![false; m];
            free.iter().enumerate().for_each(|(k, &j)| x[j] = (mask >> k) & 1 != 0);
            pivot_col.iter().enumerate().for_each(|(r, &col)| {
                let rhs = (rows[r] >> m) & 1 != 0;
                let free_sum = free.iter()
                    .filter(|&&j| rows[r] & (1 << j) != 0)
                    .fold(false, |acc, &j| acc ^ x[j]);
                x[col] = rhs ^ free_sum;
            });
            x.iter().filter(|&&b| b).count() as i64
        })
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use crate::aoc::Day;

    fn test_input() -> Vec<String> {
        vec![
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>()
    }

    #[test]
    fn test_solve_part1() {
        let day: Day = 10.try_into().unwrap();
        assert_eq!(7, day.solve_part1(&test_input()).unwrap());
    }

    #[test]
    fn test_solve_part2() {
        assert!(true)
    }
}
