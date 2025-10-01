use advent_of_code::util::{Sub, SubV2};

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i32> {
    let mut sub = Sub::default();
    for line in input.lines() {
        let cmd = line.parse().ok()?;
        sub.command(cmd);
    }
    Some(sub.hpos * sub.depth)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut sub = SubV2::default();
    for line in input.lines() {
        let cmd = line.parse().ok()?;
        sub.command(cmd);
    }
    Some(sub.hpos * sub.depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(150));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(900));
    }
}
