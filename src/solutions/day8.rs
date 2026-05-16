use anyhow::Context;

use crate::aoc::DaySolver;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &[String]) -> anyhow::Result<i64> {
        connect(input, 1000)
    }

    fn solve_part2(&self, input: &[String]) -> anyhow::Result<i64> {
        last_connection(input)
    }
}

pub(crate) fn connect(input: &[String], pairs: usize) -> anyhow::Result<i64> {
    let points = parse(input)?;
    let n = points.len();
    let mut uf = UnionFind::new(n);
    sorted_pairs(&points).into_iter().take(pairs).for_each(|(_, i, j)| { uf.union(i, j); });
    let mut sizes = uf.circuit_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    Ok(sizes.iter().take(3).product())
}

pub(crate) fn last_connection(input: &[String]) -> anyhow::Result<i64> {
    let points = parse(input)?;
    let n = points.len();
    sorted_pairs(&points)
        .into_iter()
        .scan((UnionFind::new(n), 0usize), |(uf, merges), (_, i, j)| {
            if uf.union(i, j) { *merges += 1; }
            Some((i, j, *merges))
        })
        .find(|&(_, _, count)| count == n - 1)
        .map(|(i, j, _)| points[i][0] * points[j][0])
        .context("no single circuit formed")
}

fn sorted_pairs(points: &[[i64; 3]]) -> Vec<(i64, usize, usize)> {
    let n = points.len();
    let mut pairs: Vec<_> = (0..n)
        .flat_map(|i| (i + 1..n).map(move |j| (dist2(&points[i], &points[j]), i, j)))
        .collect();
    pairs.sort_unstable();
    pairs
}

fn parse(input: &[String]) -> anyhow::Result<Vec<[i64; 3]>> {
    input
        .iter()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().context("missing x")?.trim().parse()?;
            let y = parts.next().context("missing y")?.trim().parse()?;
            let z = parts.next().context("missing z")?.trim().parse()?;
            Ok([x, y, z])
        })
        .collect()
}

fn dist2(a: &[i64; 3], b: &[i64; 3]) -> i64 {
    (0..3).map(|i| (a[i] - b[i]).pow(2)).sum()
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let (rx, ry) = (self.find(x), self.find(y));
        if rx == ry {
            return false;
        }
        let (big, small) = if self.size[rx] >= self.size[ry] { (rx, ry) } else { (ry, rx) };
        self.parent[small] = big;
        self.size[big] += self.size[small];
        true
    }

    fn circuit_sizes(&mut self) -> Vec<i64> {
        (0..self.parent.len())
            .filter_map(|i| (self.find(i) == i).then_some(self.size[i] as i64))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{connect, last_connection};
    fn test_input() -> Vec<String> {
        vec![
            "162,817,812",
            "57,618,57",
            "906,360,560",
            "592,479,940",
            "352,342,300",
            "466,668,158",
            "542,29,236",
            "431,825,988",
            "739,650,466",
            "52,470,668",
            "216,146,977",
            "819,987,18",
            "117,168,530",
            "805,96,715",
            "346,949,466",
            "970,615,88",
            "941,993,340",
            "862,61,35",
            "984,92,344",
            "425,690,689",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(40, connect(&test_input(), 10).unwrap());
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(25272, last_connection(&test_input()).unwrap());
    }
}
