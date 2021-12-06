// https://adventofcode.com/2021/day/?

pub fn part1(input: impl AsRef<str>) -> String {
    String::new()
}

pub fn part2(input: impl AsRef<str>) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_input, get_test};

    const TEST_PART_1_RESULT: &'static str = "";
    const TEST_PART_2_RESULT: &'static str = "";
    const REAL_PART_1_RESULT: &'static str = "";
    const REAL_PART_2_RESULT: &'static str = "";


    #[test]
    fn test_part1() {
        assert_eq!(part1(get_test(0)), TEST_PART_1_RESULT);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_test(0)), TEST_PART_2_RESULT);
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(part1(get_input(0)), REAL_PART_1_RESULT);
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(part2(get_input(0)), REAL_PART_2_RESULT);
    }
}
