// https://adventofcode.com/2021/day/?

fn parse_input(input: impl AsRef<str>) -> Vec<i32> {
    input
        .as_ref()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect::<Vec<_>>()
}

fn find_range(vals: &[i32]) -> (i32, i32) {
    (*vals.iter().min().unwrap(), *vals.iter().max().unwrap())
}

fn min_fule_linear(vals: &[i32]) -> i32 {
    let (min, max) = find_range(vals);

    (min..=max)
        .map(|end_location| vals.iter().map(|crab| (end_location - crab).abs()).sum())
        .min()
        .unwrap()
}

fn min_fule_exp(vals: &[i32]) -> i32 {
    let (min, max) = find_range(vals);

    (min..=max)
        .map(|end_location| {
            vals.iter()
                .map(|crab| {
                    let n = (end_location - crab).abs();
                    (n * (1 + n)) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

pub fn part1(input: impl AsRef<str>) -> String {
    let parsed_input = parse_input(input);

    min_fule_linear(&parsed_input).to_string()
}

pub fn part2(input: impl AsRef<str>) -> String {
    let parsed_input = parse_input(input);

    min_fule_exp(&parsed_input).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_input, get_test};
    use test::{black_box, Bencher};

    const DAY_NUM: u32 = 7;

    const TEST_PART_1_RESULT: &'static str = "37";
    const TEST_PART_2_RESULT: &'static str = "168";
    const REAL_PART_1_RESULT: &'static str = "352254";
    const REAL_PART_2_RESULT: &'static str = "99053143";

    #[test]
    fn test_part1() {
        let result = part1(get_test(DAY_NUM));
        assert_eq!(result, TEST_PART_1_RESULT);
        println!("Day {} Part 1 test result: {}", DAY_NUM, result);
    }

    #[test]
    fn test_part2() {
        let result = part2(get_test(DAY_NUM));
        assert_eq!(result, TEST_PART_2_RESULT);
        println!("Day {} Part 2 test result: {}", DAY_NUM, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(get_input(DAY_NUM));
        assert_eq!(result, REAL_PART_1_RESULT);
        println!("Day {} Part 1 real result: {}", DAY_NUM, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(get_input(DAY_NUM));
        assert_eq!(result, REAL_PART_2_RESULT);
        println!("Day {} Part 2 real result: {}", DAY_NUM, result);
    }

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        let input = get_input(DAY_NUM);
        b.iter(move || {
            black_box(part1(input.clone()));
        });
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        let input = get_input(DAY_NUM);
        b.iter(move || {
            black_box(part2(input.clone()));
        });
    }
}
