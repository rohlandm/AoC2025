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

    fn solve_part2(&self, input: &[String]) -> anyhow::Result<i64> {
        let points = parse(input)?;
        let (v_edges, h_edges) = edges(&points);
        let v = &v_edges;
        let h = &h_edges;

        Ok(points
            .iter()
            .enumerate()
            .flat_map(|(i, &(x1, y1))| {
                points[i + 1..].iter().filter_map(move |&(x2, y2)| {
                    let (xl, xr) = (x1.min(x2), x1.max(x2));
                    let (yb, yt) = (y1.min(y2), y1.max(y2));
                    // Rectangle is valid iff no polygon edge crosses its open interior.
                    // A vertical edge at ex crosses the interior iff xl < ex < xr
                    // and its y-range overlaps (yb, yt). Symmetric for horizontal.
                    let valid = v.iter().all(|&(ex, ey1, ey2)| {
                        !(xl < ex && ex < xr && ey1 < yt && ey2 > yb)
                    }) && h.iter().all(|&(ey, ex1, ex2)| {
                        !(yb < ey && ey < yt && ex1 < xr && ex2 > xl)
                    });
                    valid.then_some((xr - xl + 1) * (yt - yb + 1))
                })
            })
            .max()
            .unwrap_or(0))
    }
}

fn edges(points: &[(i64, i64)]) -> (Vec<(i64, i64, i64)>, Vec<(i64, i64, i64)>) {
    let n = points.len();
    let (v, h): (Vec<_>, Vec<_>) = (0..n)
        .map(|i| {
            let (xa, ya) = points[i];
            let (xb, yb) = points[(i + 1) % n];
            (xa == xb, xa, ya, xb, yb)
        })
        .partition(|&(is_vert, ..)| is_vert);
    (
        v.into_iter().map(|(_, x, ya, _, yb)| (x, ya.min(yb), ya.max(yb))).collect(),
        h.into_iter().map(|(_, xa, y, xb, _)| (y, xa.min(xb), xa.max(xb))).collect(),
    )
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
        let day: crate::aoc::Day = 9.try_into().unwrap();
        assert_eq!(24, day.solve_part2(&test_input()).unwrap());
    }
}
