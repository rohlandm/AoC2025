pub trait DaySolver {
    fn solve_part1(&self) -> Result<i64, &'static str> {
        Err("Part 1 not yet implemented")
    }
    fn solve_part2(&self) -> Result<i64, &'static str> {
        Err("Part 2 not yet implemented")
    }
}

pub struct DefaultDaySolver;

impl DaySolver for DefaultDaySolver {}
