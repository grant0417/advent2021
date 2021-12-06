// https://adventofcode.com/2021/day/6

fn parse_input(input: impl AsRef<str>) -> Vec<usize> {
    input
        .as_ref()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn compute_day(input: impl AsRef<str>, day: usize) -> usize {
    let parsed_input = parse_input(input);
    let mut fish_cohorts = [0usize; 9];

    for i in parsed_input {
        fish_cohorts[i as usize] += 1;
    }

    for _ in 0..day {
        fish_cohorts.rotate_left(1);
        fish_cohorts[6] += fish_cohorts[8];
    }

    fish_cohorts.iter().sum()
}

pub fn part1(input: impl AsRef<str>) -> String {
    compute_day(input, 80).to_string()
}

pub fn part2(input: impl AsRef<str>) -> String {
    compute_day(input, 256).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_input, get_test};

    const TEST_PART_1_RESULT: &'static str = "5934";
    const TEST_PART_2_RESULT: &'static str = "26984457539";
    const REAL_PART_1_RESULT: &'static str = "379414";
    const REAL_PART_2_RESULT: &'static str = "1705008653296";

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_test(6)), TEST_PART_1_RESULT);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_test(6)), TEST_PART_2_RESULT);
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(part1(get_input(6)), REAL_PART_1_RESULT);
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(part2(get_input(6)), REAL_PART_2_RESULT);
    }
}
