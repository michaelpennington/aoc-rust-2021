use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|l| l.parse::<i32>().unwrap())
            .tuple_windows()
            .filter(|(w1, w2)| w1 < w2)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|l| l.parse::<i32>().unwrap())
            .tuple_windows()
            .filter(|(w1, _, _, w4)| w1 < w4)
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
