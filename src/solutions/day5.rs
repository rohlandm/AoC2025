use std::str::FromStr;

use anyhow::Ok;

use crate::aoc::DaySolver;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        let (ranges, values) = parse(input);
        Ok(values
            .iter()
            .filter(|&&value| ranges.iter().any(|range| range.contains_value(value)))
            .count() as i64)
    }

    fn solve_part2(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        let (mut ranges, _) = parse(input);
        ranges.sort_by_key(|a| a.min);
        Ok(ranges
            .iter()
            .fold((0i64, 0i64), |(acc, highest), range| {
                let new = (range.max + 1 - range.min.max(highest)).max(0);
                (acc + new, highest.max(range.max + 1))
            })
            .0)
    }
}

fn parse(input: &[String]) -> (Vec<Range>, Vec<i64>) {
    let split_index = input
        .iter()
        .position(|x| x.is_empty())
        .expect("Vector should contain an empty string");

    let ranges = input[..split_index]
        .iter()
        .map(|x| x.parse().expect("all ranges should be parseable"))
        .collect();

    let values = input[split_index + 1..]
        .iter()
        .map(|x| x.parse().expect("Values should be parseable"))
        .collect();

    (ranges, values)
}

struct Range {
    min: i64,
    max: i64,
}

impl Range {
    fn contains_value(&self, value: i64) -> bool {
        (self.min..=self.max).contains(&value)
    }
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        let min = split.next().ok_or_else(|| anyhow::anyhow!("unparseable range"))?.parse()?;
        let max = split.next().ok_or_else(|| anyhow::anyhow!("unparseable range"))?.parse()?;
        Ok(Range { min, max })
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::Day;

    #[test]
    fn test_solve_part1() {
        let input = vec![
            "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let day: Day = 5.try_into().unwrap();
        assert_eq!(3, day.solve_part1(&input).unwrap());
    }

    #[test]
    fn test_solve_part2() {
        let input = vec![
            "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let day: Day = 5.try_into().unwrap();
        assert_eq!(14, day.solve_part2(&input).unwrap());
    }
}
